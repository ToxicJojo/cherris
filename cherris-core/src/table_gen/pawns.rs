use crate::{Bitboard, Square};

pub static PAWN_MOVES_WHITE: [Bitboard; Square::COUNT] = generate_pawn_moves(PAWN_PATTERN_WHITE);
pub static PAWN_MOVES_BLACK: [Bitboard; Square::COUNT] = generate_pawn_moves(PAWN_PATTERN_BLACK);
const PAWN_PATTERN_WHITE: [i8; 2] = [7, 9];
const PAWN_PATTERN_BLACK: [i8; 2] = [-7, -9];

const fn generate_pawn_moves(pattern: [i8; 2]) -> [Bitboard; Square::COUNT] {
    let mut pawn_moves = [Bitboard::EMPTY; Square::COUNT];
    let mut i = 0;

    while i < 64 {
        let pawn_move = generate_pawn_move(i as i8, pattern);
        pawn_moves[i] = pawn_move;
        i += 1;
    }

    pawn_moves
}

const fn generate_pawn_move(from: i8, pattern: [i8; 2]) -> Bitboard {
    let mut attacks = 0;
    let mut i = 0;

    while i < pattern.len() {
        let target = from + pattern[i];
        let file_from = from % 8;

        let file_target = target % 8;

        let file_diff = i8::abs(file_from - file_target);

        i += 1;

        if target < 0 || target > 63 || file_diff > 1 {
            continue;
        }

        let bb = 1 << target;
        attacks |= bb
    }

    Bitboard::new(attacks)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_white_pawn_move_a4() {
        let moves = generate_pawn_move(Square::A4.to_index() as i8, PAWN_PATTERN_WHITE);

        let squares = [Square::B5];
        let expected = Bitboard::from(squares.as_slice());

        assert_eq!(moves, expected);
    }

    #[test]
    fn generate_white_pawn_move_d4() {
        let moves = generate_pawn_move(Square::D4.to_index() as i8, PAWN_PATTERN_WHITE);

        let squares = [Square::C5, Square::E5];
        let expected = Bitboard::from(squares.as_slice());

        assert_eq!(moves, expected);
    }

    #[test]
    fn generate_white_pawn_move_h4() {
        let moves = generate_pawn_move(Square::H4.to_index() as i8, PAWN_PATTERN_WHITE);

        let squares = [Square::G5];
        let expected = Bitboard::from(squares.as_slice());

        assert_eq!(moves, expected);
    }
}
