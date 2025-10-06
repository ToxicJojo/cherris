use super::add_attacks;
use crate::{Bitboard, MoveList, Position, Role, knight_attacks};

pub fn generate_knight_moves(
    position: &Position,
    moves: &mut MoveList,
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
