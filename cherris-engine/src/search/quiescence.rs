use arrayvec::ArrayVec;
use cherris_core::{generate_loud_moves, Color, Move, Position};

use crate::{eval, move_sort::sort_moves, SearchData};

pub fn quiescence(
    alpha: i16,
    beta: i16,
    position: &Position,
    pv: &mut Vec<Move>,
    search_data: &mut SearchData,
) -> i16 {
    let mut alpha = alpha;
    search_data.nodes += 1;

    let stand_pat = match position.color_to_move {
        Color::White => eval(position),
        Color::Black => -eval(position),
    };

    if search_data.nodes > search_data.max_nodes {
        return stand_pat;
    }

    if stand_pat >= beta {
        return beta;
    }

    if alpha < stand_pat {
        alpha = stand_pat
    }

    let mut moves = ArrayVec::<Move, 256>::new();
    generate_loud_moves(position, &mut moves);

    let pv_move = search_data.pv.first();
    sort_moves(&mut moves, pv_move);
    if !search_data.pv.is_empty() {
        search_data.pv.remove(0);
    }

    for mv in moves {
        let mut local_pv = Vec::new();
        let mut next_position = *position;
        next_position.make_move(mv);
        let score = -quiescence(-beta, -alpha, &next_position, &mut local_pv, search_data);

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
