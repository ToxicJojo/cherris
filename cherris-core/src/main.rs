use std::str::FromStr;

use cherris_core::Position;

fn main() {
    let position =
        Position::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    println!("{}", position.board);
}
