use std::{str::FromStr, time::Instant};

use cherris_core::{
    generate_attacks_fast, generate_bishop_attacks, generate_bishop_xray_attacks, generate_moves,
    generate_rook_xray_attacks, Position, BISHOP_ATTACKS, BISHOP_XRAY_ATTACKS, ROOK_ATTACKS,
    ROOK_XRAY_ATTACKS,
};

fn main() {
    // TODO implement castling
    // TODO implement promotion
    // TODO make sure when the king is double checked only the king can move
    unsafe {
        ROOK_ATTACKS = generate_attacks_fast();
        ROOK_XRAY_ATTACKS = generate_rook_xray_attacks();
        BISHOP_ATTACKS = generate_bishop_attacks();
        BISHOP_XRAY_ATTACKS = generate_bishop_xray_attacks();
    };

    let mut position =
        Position::from_str("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .unwrap();

    divide(1, &mut position);
    let before = Instant::now();
    let p = perft(1, &mut position);
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
            position.make_move(mv);
            let nodes = perft(depth - 1, position);
            total += nodes;
            position.unmake_move(mv);
            println!("{}: {}", mv, nodes);
        }
    }

    println!("Total: {}", total);
}

fn perft(depth: u64, position: &mut Position) -> u64 {
    let mut nodes = 0;
    let moves = generate_moves(position);

    if depth == 1 {
        moves.len() as u64
    } else {
        for mv in moves {
            position.make_move(mv);
            nodes += perft(depth - 1, position);
            position.unmake_move(mv);
        }

        nodes
    }
}
