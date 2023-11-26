use crate::{
    Bitboard, Color, Square, BISHOP_ATTACKS, BISHOP_MASKS, BISHOP_OFFSETS, BISHOP_XRAY_ATTACKS,
    KING_MOVES, KNIGHT_MOVES, PAWN_MOVES_BLACK, PAWN_MOVES_WHITE, ROOK_ATTACKS, ROOK_MAKS,
    ROOK_OFFSETS, ROOK_XRAY_ATTACKS,
};
use std::arch::x86_64::_pext_u64;

#[inline]
pub fn pawn_attacks(square: Square, color: Color) -> Bitboard {
    match color {
        Color::White => PAWN_MOVES_WHITE[square],
        Color::Black => PAWN_MOVES_BLACK[square],
    }
}

#[inline]
pub fn king_attacks(square: Square) -> Bitboard {
    KING_MOVES[square]
}

#[inline]
pub fn knight_attacks(square: Square) -> Bitboard {
    KNIGHT_MOVES[square]
}

#[inline]
pub fn bishop_attacks(square: Square, blocker: Bitboard) -> Bitboard {
    let mask = BISHOP_MASKS[square];
    let index = unsafe { _pext_u64(blocker.0, mask) + BISHOP_OFFSETS[square] };
    unsafe { BISHOP_ATTACKS[index as usize] }
}

#[inline]
pub fn bishop_xray_attacks(square: Square, blocker: Bitboard) -> Bitboard {
    let mask = BISHOP_MASKS[square];
    let index = unsafe { _pext_u64(blocker.0, mask) + BISHOP_OFFSETS[square] };
    unsafe { BISHOP_XRAY_ATTACKS[index as usize] }
}

#[inline]
pub fn rook_attacks(square: Square, blocker: Bitboard) -> Bitboard {
    let mask = ROOK_MAKS[square];
    let index = unsafe { _pext_u64(blocker.0, mask) + ROOK_OFFSETS[square] };
    unsafe { ROOK_ATTACKS[index as usize] }
}

#[inline]
pub fn rook_xray_attacks(square: Square, blocker: Bitboard) -> Bitboard {
    let mask = ROOK_MAKS[square];
    let index = unsafe { _pext_u64(blocker.0, mask) + ROOK_OFFSETS[square] };
    unsafe { ROOK_XRAY_ATTACKS[index as usize] }
}

#[inline]
pub fn queen_attacks(square: Square, blocker: Bitboard) -> Bitboard {
    bishop_attacks(square, blocker) | rook_attacks(square, blocker)
}
