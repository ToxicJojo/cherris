use crate::{generate_moves, perft, Position};

pub fn divide(depth: u64, position: &mut Position) {
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
