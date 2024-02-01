use super::add_attacks;
use crate::{knight_attacks, Bitboard, Move, Position, Role};
use arrayvec::ArrayVec;

pub fn generate_knight_moves(
    position: &Position,
    moves: &mut ArrayVec<Move, 256>,
    hv_pins: Bitboard,
    diag_pins: Bitboard,
    check_mask: Bitboard,
) {
    let knights = position.board.role[Role::Knight] & position.board.color[position.color_to_move];
    let knights = knights & !(hv_pins | diag_pins);

    for from in knights {
        let mut attacks = knight_attacks(from);
        attacks &= !position.board.color[position.color_to_move] & check_mask;

        add_attacks(attacks, from, Role::Knight, position, moves);
    }
}
