use std::{str::FromStr, time::Instant};

use cherris_core::{generate_lookup_tables, generate_moves, perft, Position};

fn main() {
    generate_lookup_tables();

    let mut position =
        Position::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    divide(1, &mut position);
    let before = Instant::now();
    let p = perft(6, &mut position);
    println!("Elapsed time: {:.2?}", before.elapsed());
    println!("Nodes: {}", p);
}

fn divide(depth: u64, position: &mut Position) {
    let moves = generate_moves(position);
    let mut total = 0;

    for mv in moves {
        if depth == 1 {
            println!("{}: {}", mv, 1);
            total += 1;
        } else {
            let castling_rights = position.castling_rights;
            position.make_move(mv);
            let nodes = perft(depth - 1, position);
            total += nodes;
            position.unmake_move(mv);
            position.castling_rights = castling_rights;
            println!("{}: {}", mv, nodes);
        }
    }

    println!("Total: {}", total);
}
