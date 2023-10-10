use std::str::FromStr;

use cherris_core::{generate_moves, Position};

fn main() {
    let position =
        Position::from_str("r1bqkb1r/pp1ppppp/2n2n2/2p1N3/8/2N5/PPPPPPPP/R1BQKB1R w KQkq - 4 4")
            .unwrap();

    println!("{}", position.board);
    let moves = generate_moves(position);
    for mv in moves {
        println!("{}", mv);
    }
}
