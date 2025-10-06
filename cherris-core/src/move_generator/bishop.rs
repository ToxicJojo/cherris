use super::add_attacks;
use crate::{Bitboard, MoveList, Position, Role, bishop_attacks};

pub fn generate_bishop_moves(
    position: &Position,
    moves: &mut MoveList,
    hv_pins: Bitboard,
    diag_pins: Bitboard,
    check_mask: Bitboard,
    blockers: Bitboard,
) {
    let bishops = position.board.role[Role::Bishop] & position.board.color[position.color_to_move];
    let bishops = bishops & !hv_pins;
    let bishops_pinned = bishops & diag_pins;
    let bishops_unpinned = bishops ^ bishops_pinned;

    for from in bishops_unpinned {
        let mut attacks = bishop_attacks(from, blockers);
        attacks &= !position.board.color[position.color_to_move] & check_mask;

        add_attacks(attacks, from, Role::Bishop, position, moves);
    }

    for from in bishops_pinned {
        let mut attacks = bishop_attacks(from, blockers);
        attacks &= !position.board.color[position.color_to_move] & check_mask;
        attacks &= diag_pins;

        add_attacks(attacks, from, Role::Bishop, position, moves);
    }
}
