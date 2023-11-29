use std::str::FromStr;

use cherris_core::{generate_lookup_tables, generate_moves, perft, Position};

fn main() {
    generate_lookup_tables();
    println!("Hello, world!");
    let mut pos =
        Position::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    let p = perft(6, &mut pos);
    println!("{}", p);
}
