use std::str::FromStr;

use cherris_core::{Position, KNIGHT_MOVES};

fn main() {
    let position =
        Position::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    let moves = KNIGHT_MOVES[60];
    println!("{:?}", moves);
    println!("{}", position.board);
}
