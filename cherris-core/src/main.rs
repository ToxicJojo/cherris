use std::{str::FromStr, time::Instant};

use cherris_core::{
    generate_attacks_fast, generate_bishop_attacks, generate_bishop_xray_attacks, generate_moves,
    generate_rook_xray_attacks, ray_between, Color, Position, Role, BISHOP_ATTACKS,
    BISHOP_XRAY_ATTACKS, RAY_BETWEEN, ROOK_ATTACKS, ROOK_XRAY_ATTACKS,
};

fn main() {
    // TODO make sure unmaking en passant works correctly
    // TODO make sure when the king is double checked only the king can move
    unsafe {
        ROOK_ATTACKS = generate_attacks_fast();
        ROOK_XRAY_ATTACKS = generate_rook_xray_attacks();
        BISHOP_ATTACKS = generate_bishop_attacks();
        BISHOP_XRAY_ATTACKS = generate_bishop_xray_attacks();
    };

    let mut position =
        Position::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    let before = Instant::now();
    //divide(2, &mut position);
    let p = perft(6, &mut position);
    println!("Elapsed time: {:.2?}", before.elapsed());
    println!("Nodes: {}", p);

    return;
    let mut position =
        Position::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let before = Instant::now();
    for _ in 0..400 {
        let moves = generate_moves(&position);

        match moves.get(0) {
            Some(mv) => {
                //println!("Making move: {}", mv);
                position.make_move(*mv);
            }
            None => break,
        }

        println!("{}", position.board);
    }
    println!("Elapsed time: {:.2?}", before.elapsed());
    println!("{}", position.board);
}

fn divide(depth: u64, position: &mut Position) {
    let moves = generate_moves(&position);
    let mut total = 0;

    for mv in moves {
        position.make_move(mv);
        let nodes = perft(depth - 1, position);
        total += nodes;
        position.unmake_move(mv);

        println!("{}: {}", mv, nodes);
    }

    println!("Total: {}", total);
}

fn perft(depth: u64, position: &mut Position) -> u64 {
    let mut nodes = 0;
    let moves = generate_moves(&position);

    if moves.len() == 0 {
        //println!("Checkmate");
        //println!("{}", position.board);
    }

    if depth == 1 {
        return moves.len() as u64;
    } else {
        for mv in moves {
            position.make_move(mv);

            let attacked = position.board.attacked_sqaures(position.color_to_move);
            if !(attacked
                & position.board.role[Role::King]
                & position.board.color[!position.color_to_move])
                .is_empty()
            {
                println!("Illegal");
                println!("{}", position.board);
            }

            nodes += perft(depth - 1, position);
            position.unmake_move(mv);
        }

        nodes
    }
}
