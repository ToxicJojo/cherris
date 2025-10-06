use super::add_attacks;
use crate::{Bitboard, MoveList, Position, Role, king_attacks};

pub fn generate_king_moves(position: &Position, moves: &mut MoveList, attacked_squares: Bitboard) {
    let kings = position.board.role[Role::King] & position.board.color[position.color_to_move];

    for from in kings {
        let mut attacks = king_attacks(from);
        attacks &= !position.board.color[position.color_to_move];
        attacks &= !attacked_squares;

        add_attacks(attacks, from, Role::King, position, moves);
    }
}
