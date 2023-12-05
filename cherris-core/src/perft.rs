use arrayvec::ArrayVec;

use crate::{generate_moves, Move, Position};

pub fn perft(depth: u64, position: &Position) -> usize {
    let mut nodes = 0;
    let mut moves = ArrayVec::<Move, 256>::new();
    generate_moves(position, &mut moves);

    if depth == 1 {
        moves.len()
    } else {
        for mv in moves {
            let mut next_position = position.clone();
            next_position.make_move(mv);
            nodes += perft(depth - 1, &next_position);
        }

        nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_lookup_tables;
    use std::str::FromStr;

    #[test]
    fn perft_starting_pos() {
        generate_lookup_tables();

        let position =
            Position::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

        let result_1 = perft(1, &position);
        let result_2 = perft(2, &position);
        let result_3 = perft(3, &position);
        let result_4 = perft(4, &position);

        assert_eq!(result_1, 20);
        assert_eq!(result_2, 400);
        assert_eq!(result_3, 8902);
        assert_eq!(result_4, 197281);
    }

    #[test]
    fn perft_position_2() {
        generate_lookup_tables();

        let position = Position::from_str(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        )
        .unwrap();

        let result_1 = perft(1, &position);
        let result_2 = perft(2, &position);
        let result_3 = perft(3, &position);
        let result_4 = perft(4, &position);

        assert_eq!(result_1, 48);
        assert_eq!(result_2, 2039);
        assert_eq!(result_3, 97862);
        assert_eq!(result_4, 4085603);
    }

    #[test]
    fn perft_position_3() {
        generate_lookup_tables();

        let position = Position::from_str("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();

        let result_1 = perft(1, &position);
        let result_2 = perft(2, &position);
        let result_3 = perft(3, &position);
        let result_4 = perft(4, &position);

        assert_eq!(result_1, 14);
        assert_eq!(result_2, 191);
        assert_eq!(result_3, 2812);
        assert_eq!(result_4, 43238);
    }

    #[test]
    fn perft_position_4() {
        generate_lookup_tables();

        let position =
            Position::from_str("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1")
                .unwrap();

        let result_1 = perft(1, &position);
        let result_2 = perft(2, &position);
        let result_3 = perft(3, &position);
        let result_4 = perft(4, &position);

        assert_eq!(result_1, 6);
        assert_eq!(result_2, 264);
        assert_eq!(result_3, 9467);
        assert_eq!(result_4, 422333);
    }

    #[test]
    fn perft_position_5() {
        generate_lookup_tables();

        let position =
            Position::from_str("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8")
                .unwrap();

        let result_1 = perft(1, &position);
        let result_2 = perft(2, &position);
        let result_3 = perft(3, &position);
        let result_4 = perft(4, &position);

        assert_eq!(result_1, 44);
        assert_eq!(result_2, 1486);
        assert_eq!(result_3, 62379);
        assert_eq!(result_4, 2103487);
    }

    #[test]
    fn perft_position_6() {
        generate_lookup_tables();

        let position = Position::from_str(
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ",
        )
        .unwrap();

        let result_1 = perft(1, &position);
        let result_2 = perft(2, &position);
        let result_3 = perft(3, &position);
        let result_4 = perft(4, &position);

        assert_eq!(result_1, 46);
        assert_eq!(result_2, 2079);
        assert_eq!(result_3, 89890);
        assert_eq!(result_4, 3894594);
    }
}
