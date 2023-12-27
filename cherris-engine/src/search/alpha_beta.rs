use arrayvec::ArrayVec;
use cherris_core::{generate_moves, Color, Move, Position};

use crate::{eval, transposition_table::TranspositionEntryType, SearchData};

pub fn alpha_beta(
    alpha: i16,
    beta: i16,
    depth: u8,
    pv: &mut Vec<Move>,
    position: &Position,
    search_data: &mut SearchData,
) -> i16 {
    let tt_table = search_data.transposition_table.lock().unwrap();
    if let Some(tt_entry) = tt_table.get(position.zobrist) {
        if tt_entry.zobrist == position.zobrist && tt_entry.depth >= depth {
            match tt_entry.entry_type {
                TranspositionEntryType::Exact => return tt_entry.score,
                TranspositionEntryType::UpperBound => {
                    if tt_entry.score <= alpha {
                        return tt_entry.score;
                    }
                }
                TranspositionEntryType::LowerBound => {
                    if tt_entry.score >= beta {
                        return tt_entry.score;
                    }
                }
            }
        }
    }
    drop(tt_table);

    let mut moves = ArrayVec::<Move, 256>::new();
    generate_moves(position, &mut moves);

    if moves.is_empty() {
        return i16::MIN + 2;
    }

    if depth == 0 {
        match position.color_to_move {
            Color::White => return eval(position),
            Color::Black => return -eval(position),
        }
    }

    let mut alpha = alpha;

    if !search_data.pv.is_empty() {
        moves.insert(0, search_data.pv[0]);
        search_data.pv.remove(0);
    }

    let mut entry_type = TranspositionEntryType::UpperBound;

    for mv in moves {
        let mut local_pv = Vec::new();
        search_data.nodes += 1;
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
            tt_table.insert(crate::transposition_table::TranspositionEntry {
                zobrist: position.zobrist,
                score: beta,
                depth,
                entry_type: TranspositionEntryType::LowerBound,
            });
            return beta;
        }

        if score > alpha {
            entry_type = TranspositionEntryType::Exact;

            pv.clear();
            pv.push(mv);
            pv.append(&mut local_pv);
            alpha = score
        }
    }

    let mut tt_table = search_data.transposition_table.lock().unwrap();
    tt_table.insert(crate::transposition_table::TranspositionEntry {
        zobrist: position.zobrist,
        score: alpha,
        depth,
        entry_type,
    });

    alpha
}
