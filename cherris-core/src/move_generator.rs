use self::{
    bishop::generate_bishop_moves, castling::generate_castling_moves, king::generate_king_moves,
    knight::generate_knight_moves, pawn::generate_pawn_moves, queen::generate_queen_moves,
    rook::generate_rook_moves,
};
use crate::{Bitboard, Move, Position, Role, Square};
use arrayvec::ArrayVec;

mod bishop;
mod castling;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

pub fn generate_moves(position: &Position, moves: &mut ArrayVec<Move, 256>) {
    let blockers = position.board.occupied;
    let empty = !blockers;

    let attacked_squares = position.board.attacked_sqaures(!position.color_to_move);
    let check_mask = position.board.check_mask(position.color_to_move);

    let kings = position.board.role[Role::King] & position.board.color[position.color_to_move];
    let king = Square(kings.0.trailing_zeros() as u8);
    let hv_pins = position
        .board
        .horizontal_vertical_pinmask(king, !position.color_to_move);
    let diag_pins = position
        .board
        .diagonal_pinmask(king, !position.color_to_move);

    generate_pawn_moves(position, moves, empty, hv_pins, diag_pins, check_mask, king);
    generate_king_moves(position, moves, attacked_squares);
    generate_knight_moves(position, moves, hv_pins, diag_pins, check_mask);
    generate_rook_moves(position, moves, hv_pins, diag_pins, check_mask, blockers);
    generate_bishop_moves(position, moves, hv_pins, diag_pins, check_mask, blockers);
    generate_queen_moves(position, moves, hv_pins, diag_pins, check_mask, blockers);

    generate_castling_moves(position, moves, kings, blockers, attacked_squares);
}

pub fn generate_quiet_moves(position: &Position, moves: &mut ArrayVec<Move, 256>) {
    // TODO This should have custom logic to only generate non captures instead of generating all
    // moves and then filtering.
    generate_moves(position, moves);
    moves.retain(|mv| match mv {
        Move::Standard { capture, .. } => capture.is_none(),
        Move::EnPassant { .. } => false,
        _ => true,
    });
}

pub fn generate_loud_moves(position: &Position, moves: &mut ArrayVec<Move, 256>) {
    // TODO This should have custom logic to only generate captures instead of generating all
    // moves and then filtering.
    generate_moves(position, moves);
    moves.retain(|mv| match mv {
        Move::Standard { capture, .. } => capture.is_some(),
        Move::EnPassant { .. } => true,
        _ => false,
    });
}

#[inline]
fn add_attacks(
    attacks: Bitboard,
    from: Square,
    role: Role,
    position: &Position,
    moves: &mut ArrayVec<Move, 256>,
) {
    for to in attacks {
        let mv = Move::Standard {
            role,
            from,
            to,
            capture: position.board.role_on(to),
            promotion: None,
            en_passant_square: None,
        };

        unsafe {
            moves.push_unchecked(mv);
        }
    }
}

#[inline]
fn generate_promotion_move(
    from: Square,
    to: Square,
    position: &Position,
    moves: &mut ArrayVec<Move, 256>,
) {
    for role in Role::iter().skip(1).take(4) {
        let mv = Move::Standard {
            role: Role::Pawn,
            from,
            to,
            capture: position.board.role_on(to),
            promotion: Some(*role),
            en_passant_square: None,
        };

        unsafe {
            moves.push_unchecked(mv);
        }
    }
}
