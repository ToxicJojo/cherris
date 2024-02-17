use cherris_core::{Color, Move, MoveList};

use crate::{history::HistoryTable, ROLE_VALUE};

pub fn sort_moves(
    moves: &mut MoveList,
    tt_move: Option<Move>,
    history: &HistoryTable,
    color: Color,
) {
    moves.sort_by_key(|mv| score_move(mv, tt_move, history, color))
}

fn score_move(mv: &Move, tt_move: Option<Move>, history: &HistoryTable, color: Color) -> i16 {
    if let Some(tt_move) = tt_move {
        if mv == &tt_move {
            return -10000;
        }
    }

    match mv {
        Move::Standard { role, capture, .. } => match capture {
            Some(capture) => ROLE_VALUE[role] - ROLE_VALUE[capture],
            None => -history.get(color, *mv),
        },
        Move::EnPassant { .. } => 0,
        Move::CastleShort => 1000,
        Move::CastleLong => 1000,
    }
}
