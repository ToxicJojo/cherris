use crate::{
    Bitboard, Color, Square, BISHOP_ATTACKS, BISHOP_MASKS, BISHOP_OFFSETS, BISHOP_XRAY_ATTACKS,
    KING_MOVES, KNIGHT_MOVES, PAWN_MOVES_BLACK, PAWN_MOVES_WHITE, ROOK_ATTACKS, ROOK_MAKS,
    ROOK_OFFSETS, ROOK_XRAY_ATTACKS,
};
use std::arch::x86_64::_pext_u64;

#[inline]
pub fn pawn_attacks(square: Square, color: Color) -> Bitboard {
    unsafe {
        match color {
            Color::White => *PAWN_MOVES_WHITE.get_unchecked(square.to_index()),
            Color::Black => *PAWN_MOVES_BLACK.get_unchecked(square.to_index()),
        }
    }
}

#[inline]
pub fn king_attacks(square: Square) -> Bitboard {
    unsafe { *KING_MOVES.get_unchecked(square.to_index()) }
}

#[inline]
pub fn knight_attacks(square: Square) -> Bitboard {
    unsafe { *KNIGHT_MOVES.get_unchecked(square.to_index()) }
}

#[inline]
pub fn bishop_attacks(square: Square, blocker: Bitboard) -> Bitboard {
    unsafe {
        let mask = *BISHOP_MASKS.get_unchecked(square.to_index());
        let index = _pext_u64(blocker.0, mask) + *BISHOP_OFFSETS.get_unchecked(square.to_index());
        *BISHOP_ATTACKS.get_unchecked(index as usize)
    }
}

#[inline]
pub fn bishop_xray_attacks(square: Square, blocker: Bitboard) -> Bitboard {
    unsafe {
        let mask = *BISHOP_MASKS.get_unchecked(square.to_index());
        let index = _pext_u64(blocker.0, mask) + *BISHOP_OFFSETS.get_unchecked(square.to_index());
        *BISHOP_XRAY_ATTACKS.get_unchecked(index as usize)
    }
}

#[inline]
pub fn rook_attacks(square: Square, blocker: Bitboard) -> Bitboard {
    unsafe {
        let mask = *ROOK_MAKS.get_unchecked(square.to_index());
        let index = _pext_u64(blocker.0, mask) + *ROOK_OFFSETS.get_unchecked(square.to_index());
        *ROOK_ATTACKS.get_unchecked(index as usize)
    }
}

#[inline]
pub fn rook_xray_attacks(square: Square, blocker: Bitboard) -> Bitboard {
    unsafe {
        let mask = *ROOK_MAKS.get_unchecked(square.to_index());
        let index = _pext_u64(blocker.0, mask) + *ROOK_OFFSETS.get_unchecked(square.to_index());
        *ROOK_XRAY_ATTACKS.get_unchecked(index as usize)
    }
}

#[inline]
pub fn queen_attacks(square: Square, blocker: Bitboard) -> Bitboard {
    bishop_attacks(square, blocker) | rook_attacks(square, blocker)
}
