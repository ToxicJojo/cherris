use std::{str::FromStr, time::Instant};

use cherris_core::{
    generate_attacks_fast, generate_bishop_attacks, generate_moves, Position, BISHOP_ATTACKS,
    ROOK_ATTACKS,
};

fn main() {
    unsafe {
        ROOK_ATTACKS = generate_attacks_fast();
        BISHOP_ATTACKS = generate_bishop_attacks();
    };

    let mut position =
        Position::from_str("r1bqkb1r/pp1ppppp/2n2n2/2p1N3/8/2N5/PPP1PPPP/R1BQKB1R w KQkq - 4 6")
            .unwrap();

    println!("{}", position.board);

    let before = Instant::now();
    for _ in 0..4000000 {
        let moves = generate_moves(&position);

        match moves.get(0) {
            Some(mv) => {
                //println!("Making move: {}", mv);
                position.make_move(*mv);
            }
            None => break,
        }

        //println!("{}", position.board);
    }
    println!("Elapsed time: {:.2?}", before.elapsed());
    println!("{}", position.board);
}
