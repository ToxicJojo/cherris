use arrayvec::ArrayVec;
use cherris_core::{generate_moves, Move, Position};

use crate::eval;

pub fn alpha_beta_max(alpha: i16, beta: i16, depth: u8, position: &Position) -> i16 {
    let mut moves = ArrayVec::<Move, 256>::new();
    generate_moves(position, &mut moves);

    if moves.is_empty() {
        return i16::MIN;
    }

    let mut alpha = alpha;
    if depth == 0 {
        return eval(position);
    }

    for mv in moves {
        let mut next_position = *position;
        next_position.make_move(mv);
        let score = alpha_beta_min(alpha, beta, depth - 1, &next_position);

        if score >= beta {
            return beta;
        }

        if score > alpha {
            alpha = score
        }
    }

    alpha
}

pub fn alpha_beta_min(alpha: i16, beta: i16, depth: u8, position: &Position) -> i16 {
    let mut moves = ArrayVec::<Move, 256>::new();
    generate_moves(position, &mut moves);

    if moves.is_empty() {
        return i16::MAX;
    }

    let mut beta = beta;
    if depth == 0 {
        return eval(position);
    }

    for mv in moves {
        let mut next_position = *position;
        next_position.make_move(mv);
        let score = alpha_beta_max(alpha, beta, depth - 1, &next_position);

        if score <= alpha {
            return alpha;
        }

        if score < beta {
            beta = score
        }
    }

    beta
}
