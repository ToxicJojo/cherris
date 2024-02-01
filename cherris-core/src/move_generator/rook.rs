use super::add_attacks;
use crate::{rook_attacks, Bitboard, Move, Position, Role};
use arrayvec::ArrayVec;

pub fn generate_rook_moves(
    position: &Position,
    moves: &mut ArrayVec<Move, 256>,
    hv_pins: Bitboard,
    diag_pins: Bitboard,
    check_mask: Bitboard,
    blockers: Bitboard,
) {
    let rooks = position.board.role[Role::Rook] & position.board.color[position.color_to_move];
    let rooks = rooks & !diag_pins;
    let rooks_pinned = rooks & hv_pins;
    let rooks_unpinned = rooks ^ rooks_pinned;

    for from in rooks_unpinned {
        let mut attacks = rook_attacks(from, blockers);
        attacks &= !position.board.color[position.color_to_move] & check_mask;

        add_attacks(attacks, from, Role::Rook, position, moves);
    }

    for from in rooks_pinned {
        let mut attacks = rook_attacks(from, blockers);
        attacks &= !position.board.color[position.color_to_move] & check_mask;
        attacks &= hv_pins;

        add_attacks(attacks, from, Role::Rook, position, moves);
    }
}
