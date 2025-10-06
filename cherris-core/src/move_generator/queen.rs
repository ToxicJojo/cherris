use super::add_attacks;
use crate::{Bitboard, MoveList, Position, Role, bishop_attacks, queen_attacks, rook_attacks};

pub fn generate_queen_moves(
    position: &Position,
    moves: &mut MoveList,
    hv_pins: Bitboard,
    diag_pins: Bitboard,
    check_mask: Bitboard,
    blockers: Bitboard,
) {
    let queens = position.board.role[Role::Queen] & position.board.color[position.color_to_move];
    let queens_pinned_hv = queens & hv_pins;
    let queens_pinned_diag = queens & diag_pins;
    let queens_unpinned = queens ^ (queens_pinned_hv | queens_pinned_diag);

    for from in queens_unpinned {
        let mut attacks = queen_attacks(from, blockers);
        attacks &= !position.board.color[position.color_to_move] & check_mask;

        add_attacks(attacks, from, Role::Queen, position, moves);
    }

    for from in queens_pinned_diag {
        let mut attacks = bishop_attacks(from, blockers);
        attacks &= !position.board.color[position.color_to_move] & check_mask & diag_pins;

        add_attacks(attacks, from, Role::Queen, position, moves);
    }

    for from in queens_pinned_hv {
        let mut attacks = rook_attacks(from, blockers);
        attacks &= !position.board.color[position.color_to_move] & check_mask & hv_pins;

        add_attacks(attacks, from, Role::Queen, position, moves);
    }
}
