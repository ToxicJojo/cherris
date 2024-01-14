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
    unsafe {
        let mask = BISHOP_MASKS[square];
        let index = _pext_u64(blocker.0, mask) + BISHOP_OFFSETS[square];
        *BISHOP_ATTACKS.get_unchecked(index as usize)
    }
}

#[inline]
pub fn bishop_xray_attacks(square: Square, blocker: Bitboard) -> Bitboard {
    unsafe {
        let mask = BISHOP_MASKS[square];
        let index = _pext_u64(blocker.0, mask) + BISHOP_OFFSETS[square];
        *BISHOP_XRAY_ATTACKS.get_unchecked(index as usize)
    }
}

#[inline]
pub fn rook_attacks(square: Square, blocker: Bitboard) -> Bitboard {
    unsafe {
        let mask = ROOK_MAKS[square];
        let index = _pext_u64(blocker.0, mask) + ROOK_OFFSETS[square];
        *ROOK_ATTACKS.get_unchecked(index as usize)
    }
}

#[inline]
pub fn rook_xray_attacks(square: Square, blocker: Bitboard) -> Bitboard {
    unsafe {
        let mask = ROOK_MAKS[square];
        let index = _pext_u64(blocker.0, mask) + ROOK_OFFSETS[square];
        *ROOK_XRAY_ATTACKS.get_unchecked(index as usize)
    }
}

#[inline]
pub fn queen_attacks(square: Square, blocker: Bitboard) -> Bitboard {
    bishop_attacks(square, blocker) | rook_attacks(square, blocker)
}

#[cfg(test)]
mod tests {
    use crate::knight_attacks;

    use super::*;

    #[test]
    fn pawn_attacks_white() {
        let attacks_center = pawn_attacks(Square::E4, Color::White);
        let attacks_edge_left = pawn_attacks(Square::A4, Color::White);
        let attacks_edge_right = pawn_attacks(Square::H4, Color::White);

        let expected_center = Bitboard::from([Square::D5, Square::F5].as_slice());
        let expected_edge_left = Bitboard::from(Square::B5);
        let expected_edge_right = Bitboard::from(Square::G5);

        assert_eq!(expected_center, attacks_center);
        assert_eq!(expected_edge_left, attacks_edge_left);
        assert_eq!(expected_edge_right, attacks_edge_right);
    }

    #[test]
    fn pawn_attacks_black() {
        let attacks_center = pawn_attacks(Square::E6, Color::Black);
        let attacks_edge_left = pawn_attacks(Square::A6, Color::Black);
        let attacks_edge_right = pawn_attacks(Square::H6, Color::Black);

        let expected_center = Bitboard::from([Square::D5, Square::F5].as_slice());
        let expected_edge_left = Bitboard::from(Square::B5);
        let expected_edge_right = Bitboard::from(Square::G5);

        assert_eq!(expected_center, attacks_center);
        assert_eq!(expected_edge_left, attacks_edge_left);
        assert_eq!(expected_edge_right, attacks_edge_right);
    }

    #[test]
    fn knight_attacks_test() {
        let attacks_a1 = knight_attacks(Square::A1);
        let attacks_d4 = knight_attacks(Square::D4);
        let attacks_h8 = knight_attacks(Square::H8);

        let expected_a1 = Bitboard::from([Square::B3, Square::C2].as_slice());
        let expected_d4 = Bitboard::from(
            [
                Square::E6,
                Square::F5,
                Square::F3,
                Square::E2,
                Square::C2,
                Square::B3,
                Square::B5,
                Square::C6,
            ]
            .as_slice(),
        );
        let expected_h8 = Bitboard::from([Square::F7, Square::G6].as_slice());

        assert_eq!(attacks_a1, expected_a1);
        assert_eq!(attacks_d4, expected_d4);
        assert_eq!(attacks_h8, expected_h8);
    }

    #[test]
    fn king_attacks_test() {
        let attacks_a1 = king_attacks(Square::A1);
        let attacks_d4 = king_attacks(Square::D4);
        let attacks_h8 = king_attacks(Square::H8);

        let expected_a1 = Bitboard::from([Square::A2, Square::B2, Square::B1].as_slice());
        let expected_d4 = Bitboard::from(
            [
                Square::D5,
                Square::E5,
                Square::E4,
                Square::E3,
                Square::D3,
                Square::C3,
                Square::C4,
                Square::C5,
            ]
            .as_slice(),
        );

        let expected_h8 = Bitboard::from([Square::H7, Square::G7, Square::G8].as_slice());

        assert_eq!(attacks_a1, expected_a1);
        assert_eq!(attacks_d4, expected_d4);
        assert_eq!(attacks_h8, expected_h8);
    }
}
