use cherris_core::{generate_moves, Move, MoveList, Position};

use crate::{
    move_sort::sort_moves,
    quiescence::quiescence,
    transposition_table::{TranspositionEntry, TranspositionEntryType},
    SearchData,
};

pub fn alpha_beta(
    alpha: i16,
    beta: i16,
    depth: u8,
    pv: &mut Vec<Move>,
    position: &Position,
    search_data: &mut SearchData,
) -> i16 {
    let is_root = search_data.nodes == 0;

    let tt_table = search_data.transposition_table.lock().unwrap();
    let (tt_move, tt_value) = tt_table.get(position, alpha, beta, depth);
    drop(tt_table);

    if let Some(tt_value) = tt_value {
        if !is_root {
            return tt_value;
        }
    }

    let mut moves = MoveList::new();
    generate_moves(position, &mut moves);

    if moves.is_empty() {
        if position.is_in_check() {
            return i16::MIN + 6;
        } else {
            return 0;
        }
    }

    if depth == 0 {
        return quiescence(alpha, beta, position, pv, search_data);
    }

    let mut alpha = alpha;

    let mut entry_type = TranspositionEntryType::UpperBound;

    let history_table = search_data.history_table.lock().unwrap();
    sort_moves(&mut moves, tt_move, &history_table, position.color_to_move);
    drop(history_table);

    let mut best_move = *moves.first().unwrap();
    for mv in moves {
        search_data.nodes += 1;

        if search_data.nodes > search_data.max_nodes {
            break;
        }

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
