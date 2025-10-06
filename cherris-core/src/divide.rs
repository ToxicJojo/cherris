use crate::{MoveList, Position, generate_moves, perft};

pub fn divide(depth: u64, position: &Position) {
    let mut moves = MoveList::new();
    generate_moves(position, &mut moves);
    let mut total = 0;

    for mv in moves {
        if depth == 1 {
            println!("{}: {}", mv, 1);
            total += 1;
        } else {
            let mut next_position = *position;
            next_position.make_move(mv);

            let nodes = perft(depth - 1, &next_position);
            total += nodes;
            println!("{}: {}", mv, nodes);
        }
    }

    println!("Total: {}", total);
}
