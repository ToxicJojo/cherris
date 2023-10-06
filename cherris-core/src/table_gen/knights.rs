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
        let rank_from = from / 8;
        let file_from = from % 8;

        let rank_target = target / 8;
        let file_target = target % 8;

        let rank_diff = i8::abs(rank_from - rank_target);
        let file_diff = i8::abs(file_from - file_target);

        i += 1;

        if target < 0 || target > 63 || rank_diff > 2 || file_diff > 2 {
            continue;
        }

        let bb = 1 << target;
        attacks |= bb
    }

    Bitboard::new(attacks)
}
