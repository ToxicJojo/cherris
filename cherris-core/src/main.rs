use std::str::FromStr;

use cherris_core::{Position, KING_MOVES};

fn main() {
    let position =
        Position::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    let moves = KING_MOVES[26];
    println!("{:?}", moves);
    println!("{}", position.board);
}
