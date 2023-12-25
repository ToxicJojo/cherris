use arrayvec::ArrayVec;
use cherris_core::{generate_moves, Color, Move, Position};

use crate::{eval, SearchData};

pub fn alpha_beta(
    alpha: i16,
    beta: i16,
    depth: u8,
    pv: &mut Vec<Move>,
    position: &Position,
    search_data: &mut SearchData,
) -> i16 {
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
