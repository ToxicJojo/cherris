use arrayvec::ArrayVec;
use cherris_core::Move;

use crate::ROLE_VALUE;

pub fn sort_moves(moves: &mut ArrayVec<Move, 256>, pv_move: Option<&Move>) {
    moves.sort_unstable_by_key(|mv| score_move(mv, pv_move))
}

fn score_move(mv: &Move, pv_move: Option<&Move>) -> i16 {
    if let Some(pv_move) = pv_move {
        if mv == pv_move {
            return -10000;
        }
    }

    match mv {
        Move::Standard { role, capture, .. } => match capture {
            Some(capture) => ROLE_VALUE[role] - ROLE_VALUE[capture],
            None => 1000,
        },
        Move::EnPassant { .. } => 0,
        Move::CastleShort => 1000,
        Move::CastleLong => 1000,
    }
}
