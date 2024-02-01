use super::add_attacks;
use crate::{king_attacks, Bitboard, Move, Position, Role};
use arrayvec::ArrayVec;

pub fn generate_king_moves(
    position: &Position,
    moves: &mut ArrayVec<Move, 256>,
    attacked_squares: Bitboard,
) {
    let kings = position.board.role[Role::King] & position.board.color[position.color_to_move];

    for from in kings {
        let mut attacks = king_attacks(from);
        attacks &= !position.board.color[position.color_to_move];
        attacks &= !attacked_squares;

        add_attacks(attacks, from, Role::King, position, moves);
    }
}
