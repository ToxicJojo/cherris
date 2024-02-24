use cherris_core::{Move, MoveList};

use crate::ROLE_VALUE;

pub fn sort_moves(moves: &mut MoveList, tt_move: Option<Move>) {
    moves.sort_by_key(|mv| score_move(mv, tt_move))
}

fn score_move(mv: &Move, tt_move: Option<Move>) -> i16 {
    if let Some(tt_move) = tt_move {
        if mv == &tt_move {
            return -10000;
        }
    }

    match mv {
        Move::Standard { role, capture, .. } => match capture {
            Some(capture) => ROLE_VALUE[role] - ROLE_VALUE[capture],
            None => 0,
        },
        Move::EnPassant { .. } => 0,
        Move::CastleShort => 1000,
        Move::CastleLong => 1000,
    }
}
