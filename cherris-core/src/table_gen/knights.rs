use crate::{Bitboard, Square};

pub static KNIGHT_MOVES: [Bitboard; Square::COUNT] = generate_knight_moves();
const KNIGHT_PATTERN: [i8; 8] = [17, 10, -6, -15, -17, -10, 6, 15];

const fn generate_knight_moves() -> [Bitboard; Square::COUNT] {
    let mut knight_moves = [Bitboard::EMPTY; Square::COUNT];
    let mut i = 0;

    while i < 64 {
        let knight_move = generate_knight_move(i as i8);
        knight_moves[i] = knight_move;
        i += 1;
    }

    knight_moves
}

const fn generate_knight_move(from: i8) -> Bitboard {
    let mut attacks = 0;
    let mut i = 0;

    while i < KNIGHT_PATTERN.len() {
        let target = from + KNIGHT_PATTERN[i];
        let file_from = from % 8;
        let file_target = target % 8;

        let file_diff = i8::abs(file_from - file_target);

        i += 1;

        if target < 0 || target > 63 || file_diff > 2 {
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
    fn generate_knight_move_a1() {
        let attacks = generate_knight_move(0);

        let squares = [Square::B3, Square::C2];
        let expected = Bitboard::from(squares.as_slice());

        assert_eq!(attacks, expected);
    }

    #[test]
    fn generate_knight_move_d4() {
        let attacks = generate_knight_move(Square::D4.to_index() as i8);

        let squares = [
            Square::E6,
            Square::F5,
            Square::F3,
            Square::E2,
            Square::C2,
            Square::B3,
            Square::B5,
            Square::C6,
        ];
        let expected = Bitboard::from(squares.as_slice());

        assert_eq!(attacks, expected);
    }

    #[test]
    fn generate_knight_move_h8() {
        let attacks = generate_knight_move(63);

        let squares = [Square::F7, Square::G6];
        let expected = Bitboard::from(squares.as_slice());

        assert_eq!(attacks, expected);
    }
}
