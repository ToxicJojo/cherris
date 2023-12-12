use arrayvec::ArrayVec;
use cherris_core::{generate_moves, Move, Position};

use crate::{eval, SearchData};

pub fn alpha_beta_max(
    alpha: i16,
    beta: i16,
    depth: u8,
    pv: &mut Vec<Move>,
    position: &Position,
    search_data: &mut SearchData,
) -> i16 {
    if depth == 0 {
        return eval(position);
    }

    let mut moves = ArrayVec::<Move, 256>::new();
    generate_moves(position, &mut moves);

    if moves.is_empty() {
        return i16::MIN;
    }

    let mut alpha = alpha;

    for mv in moves {
        let mut local_pv = Vec::new();
        search_data.nodes += 1;
        let mut next_position = *position;
        next_position.make_move(mv);
        let score = alpha_beta_min(
            alpha,
            beta,
            depth - 1,
            &mut local_pv,
            &next_position,
            search_data,
        );

        if score >= beta {
            return beta;
        }

        if score > alpha {
            pv.clear();
            pv.push(mv);
            pv.append(&mut local_pv);
            alpha = score
        }
    }

    alpha
}

pub fn alpha_beta_min(
    alpha: i16,
    beta: i16,
    depth: u8,
    pv: &mut Vec<Move>,
    position: &Position,
    search_data: &mut SearchData,
) -> i16 {
    if depth == 0 {
        return eval(position);
    }

    let mut moves = ArrayVec::<Move, 256>::new();
    generate_moves(position, &mut moves);

    if moves.is_empty() {
        return i16::MAX;
    }

    let mut beta = beta;

    for mv in moves {
        let mut local_pv = Vec::new();
        search_data.nodes += 1;
        let mut next_position = *position;
        next_position.make_move(mv);
        let score = alpha_beta_max(
            alpha,
            beta,
            depth - 1,
            &mut local_pv,
            &next_position,
            search_data,
        );

        if score <= alpha {
            return alpha;
        }

        if score < beta {
            pv.clear();
            pv.push(mv);
            pv.append(&mut local_pv);

            beta = score
        }
    }

    beta
}
