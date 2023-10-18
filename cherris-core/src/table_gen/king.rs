use crate::{Bitboard, Square};

pub static KING_MOVES: [Bitboard; Square::COUNT] = generate_king_moves();
const KING_PATTERN: [i8; 8] = [1, -1, 7, -7, 8, -8, 9, -9];

const fn generate_king_moves() -> [Bitboard; Square::COUNT] {
    let mut king_moves = [Bitboard::EMPTY; Square::COUNT];
    let mut i = 0;

    while i < 64 {
        let king_move = generate_king_move(i as i8);
        king_moves[i] = king_move;
        i += 1;
    }

    king_moves
}

const fn generate_king_move(from: i8) -> Bitboard {
    let mut attacks = 0;
    let mut i = 0;

    while i < KING_PATTERN.len() {
        let target = from + KING_PATTERN[i];
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
    fn generate_king_move_a1() {
        let attacks = generate_king_move(0);

        let squares = [Square::A2, Square::B2, Square::B1];
        let expected = Bitboard::from(squares.as_slice());

        assert_eq!(attacks, expected);
    }

    #[test]
    fn generate_king_move_d4() {
        let attacks = generate_king_move(Square::D4.to_index() as i8);

        let squares = [
            Square::D5,
            Square::E5,
            Square::E4,
            Square::E3,
            Square::D3,
            Square::C3,
            Square::C4,
            Square::C5,
        ];
        let expected = Bitboard::from(squares.as_slice());

        assert_eq!(attacks, expected);
    }

    #[test]
    fn generate_king_move_h8() {
        let attacks = generate_king_move(63);

        let squares = [Square::H7, Square::G7, Square::G8];
        let expected = Bitboard::from(squares.as_slice());

        assert_eq!(attacks, expected);
    }
}
