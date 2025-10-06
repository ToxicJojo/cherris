use cherris_core::{Move, MoveList, Position, generate_moves};

use crate::{
    SearchData,
    evaluation::Evaluation,
    move_sort::sort_moves,
    quiescence::quiescence,
    transposition_table::{TranspositionEntry, TranspositionEntryType},
};

pub fn alpha_beta(
    alpha: Evaluation,
    beta: Evaluation,
    depth: u8,
    pv: &mut Vec<Move>,
    position: &Position,
    search_data: &mut SearchData,
) -> Evaluation {
    let is_root = search_data.nodes == 0;

    let tt_table = search_data.transposition_table.lock().unwrap();
    let (tt_move, tt_value) = tt_table.get(position, alpha, beta, depth);
    drop(tt_table);

    if let Some(tt_value) = tt_value {
        if !is_root {
            return tt_value;
        }
    }

    let is_in_check = position.is_in_check();

    if depth == 0 && !is_in_check {
        return quiescence(alpha, beta, position, pv, search_data);
    }

    let mut moves = MoveList::new();
    generate_moves(position, &mut moves);

    if moves.is_empty() {
        if is_in_check {
            return Evaluation::new_mate_in(search_data.current_depth);
        } else {
            return Evaluation::DRAW;
        }
    }

    if depth == 0 {
        return quiescence(alpha, beta, position, pv, search_data);
    }

    let mut alpha = alpha;

    let mut entry_type = TranspositionEntryType::UpperBound;

    sort_moves(&mut moves, tt_move);

    let mut best_move = *moves.first().unwrap();
    for mv in moves {
        search_data.nodes += 1;

        if search_data.nodes > search_data.max_nodes {
            break;
        }

        search_data.current_depth += 1;

        let mut local_pv = Vec::new();
        let mut next_position = *position;
        next_position.make_move(mv);
        let score = -alpha_beta(
            -beta,
            -alpha,
            depth - 1,
            &mut local_pv,
            &next_position,
            search_data,
        );
        search_data.current_depth -= 1;

        if score >= beta {
            let mut tt_table = search_data.transposition_table.lock().unwrap();
            tt_table.insert(TranspositionEntry {
                zobrist: position.zobrist,
                score: beta,
                depth,
                entry_type: TranspositionEntryType::LowerBound,
                chess_move: mv,
            });

            return beta;
        }

        if score > alpha {
            entry_type = TranspositionEntryType::Exact;

            pv.clear();
            pv.push(mv);
            pv.append(&mut local_pv);
            best_move = mv;
            alpha = score
        }
    }

    let mut tt_table = search_data.transposition_table.lock().unwrap();
    tt_table.insert(TranspositionEntry {
        zobrist: position.zobrist,
        score: alpha,
        depth,
        entry_type,
        chess_move: best_move,
    });

    alpha
}
