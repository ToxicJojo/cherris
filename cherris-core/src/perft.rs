use crate::{MoveList, Position, generate_moves};

pub fn perft(depth: u64, position: &Position) -> usize {
    let mut nodes = 0;
    let mut moves = MoveList::new();
    generate_moves(position, &mut moves);

    if depth == 1 {
        moves.len()
    } else {
        for mv in moves {
            let mut next_position = *position;
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

    fn test_edp(edp: &str) {
        generate_lookup_tables();
        let parts: Vec<&str> = edp.split(';').collect();
        let fen = parts[0];
        let position = Position::from_str(fen).unwrap();

        for depth in parts.iter().skip(1) {
            let parts: Vec<&str> = depth.split(' ').collect();
            let depth = &parts[0][1..];
            let depth = u64::from_str(depth).unwrap();
            let expected = parts[1];
            let expected = usize::from_str(expected).unwrap();

            let result = perft(depth, &position);
            assert_eq!(expected, result);
        }
    }

    #[test]
    fn perft_starting_pos() {
        test_edp(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ;D1 20 ;D2 400 ;D3 8902 ;D4 197281",
        );
    }

    #[test]
    fn perft_position_2() {
        test_edp(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ;D1 48 ;D2 2039 ;D3 97862 ;D4 4085603",
        );
    }

    #[test]
    fn perft_position_3() {
        test_edp("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1 ;D1 14 ;D2 191 ;D3 2812 ;D4 43238");
    }

    #[test]
    fn perft_position_4() {
        test_edp(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1 ;D1 6 ;D2 264 ;D3 9467 ;D4 422333",
        );
    }
    #[test]
    fn perft_position_5() {
        test_edp(
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8 ;D1 44 ;D2 1486 ;D3 62379 ;D4 2103487",
        );
    }

    #[test]
    fn perft_position_6() {
        test_edp(
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ;D1 46 ;D2 2079 ;D3 89890 ;D4 3894594",
        );
    }

    #[test]
    #[ignore]
    fn tricky() {
        let edps = [
            "3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1 ;D6 1134888",
            "r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1 ;D4 1274206",
            "8/8/8/8/k1p4R/8/3P4/3K4 w - - 0 1 ;D6 1134888",
            "8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1 ;D6 1440467",
            "8/5k2/8/2Pp4/2B5/1K6/8/8 w - d6 0 1 ;D6 1440467",
            "8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1 ;D6 1015133",
            "8/b2p2k1/8/2P5/8/4K3/8/8 b - - 0 1 ;D6 1015133",
            "5k2/8/8/8/8/8/8/4K2R w K - 0 1 ;D6 661072",
            "4k2r/8/8/8/8/8/8/5K2 b k - 0 1 ;D6 661072",
            "3k4/8/8/8/8/8/8/R3K3 w Q - 0 1 ;D6 803711",
            "r3k3/8/8/8/8/8/8/3K4 b q - 0 1 ;D6 803711",
            "K1k5/8/P7/8/8/8/8/8 w - - 0 1 ;D6 2217",
            "8/8/8/8/8/p7/8/k1K5 b - - 0 1 ;D6 2217",
            "8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1 ;D6 1440467",
            "8/5k2/8/2Pp4/2B5/1K6/8/8 w - d6 0 1 ;D6 1440467",
            "3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1 ;D6 1134888",
            "8/8/8/8/k1p4R/8/3P4/3K4 w - - 0 1 ;D6 1134888",
            "8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1 ;D6 1015133",
            "8/b2p2k1/8/2P5/8/4K3/8/8 b - - 0 1 ;D6 1015133",
        ];

        for edp in edps {
            test_edp(edp);
        }
    }

    #[test]
    #[ignore]
    fn large_suite() {
        let edps = [
            "k7/8/3p4/8/3P4/8/8/7K b - - 0 1 ;D1 4 ;D2 15 ;D3 89 ;D4 537 ;D5 3309 ;D6 21104",
            "7k/3p4/8/8/3P4/8/8/K7 w - - 0 1 ;D1 4 ;D2 19 ;D3 117 ;D4 720 ;D5 4661 ;D6 32191",
            "7k/8/8/3p4/8/8/3P4/K7 w - - 0 1 ;D1 5 ;D2 19 ;D3 116 ;D4 716 ;D5 4786 ;D6 30980",
            "k7/8/8/7p/6P1/8/8/K7 w - - 0 1 ;D1 5 ;D2 22 ;D3 139 ;D4 877 ;D5 6112 ;D6 41874",
            "k7/8/7p/8/8/6P1/8/K7 w - - 0 1 ;D1 4 ;D2 16 ;D3 101 ;D4 637 ;D5 4354 ;D6 29679",
            "k7/8/8/6p1/7P/8/8/K7 w - - 0 1 ;D1 5 ;D2 22 ;D3 139 ;D4 877 ;D5 6112 ;D6 41874",
            "k7/8/6p1/8/8/7P/8/K7 w - - 0 1 ;D1 4 ;D2 16 ;D3 101 ;D4 637 ;D5 4354 ;D6 29679",
            "k7/8/8/3p4/4p3/8/8/7K w - - 0 1 ;D1 3 ;D2 15 ;D3 84 ;D4 573 ;D5 3013 ;D6 22886",
            "k7/8/3p4/8/8/4P3/8/7K w - - 0 1 ;D1 4 ;D2 16 ;D3 101 ;D4 637 ;D5 4271 ;D6 28662",
            "7k/3p4/8/8/3P4/8/8/K7 b - - 0 1 ;D1 5 ;D2 19 ;D3 117 ;D4 720 ;D5 5014 ;D6 32167",
            "7k/8/8/3p4/8/8/3P4/K7 b - - 0 1 ;D1 4 ;D2 19 ;D3 117 ;D4 712 ;D5 4658 ;D6 30749",
            "k7/8/8/7p/6P1/8/8/K7 b - - 0 1 ;D1 5 ;D2 22 ;D3 139 ;D4 877 ;D5 6112 ;D6 41874",
            "k7/8/7p/8/8/6P1/8/K7 b - - 0 1 ;D1 4 ;D2 16 ;D3 101 ;D4 637 ;D5 4354 ;D6 29679",
            "k7/8/8/6p1/7P/8/8/K7 b - - 0 1 ;D1 5 ;D2 22 ;D3 139 ;D4 877 ;D5 6112 ;D6 41874",
            "k7/8/6p1/8/8/7P/8/K7 b - - 0 1 ;D1 4 ;D2 16 ;D3 101 ;D4 637 ;D5 4354 ;D6 29679",
            "k7/8/8/3p4/4p3/8/8/7K b - - 0 1 ;D1 5 ;D2 15 ;D3 102 ;D4 569 ;D5 4337 ;D6 22579",
            "k7/8/3p4/8/8/4P3/8/7K b - - 0 1 ;D1 4 ;D2 16 ;D3 101 ;D4 637 ;D5 4271 ;D6 28662",
            "7k/8/8/p7/1P6/8/8/7K w - - 0 1 ;D1 5 ;D2 22 ;D3 139 ;D4 877 ;D5 6112 ;D6 41874",
            "7k/8/8/p7/1P6/8/8/7K b - - 0 1 ;D1 5 ;D2 22 ;D3 139 ;D4 877 ;D5 6112 ;D6 41874",
            "7k/8/8/1p6/P7/8/8/7K w - - 0 1 ;D1 5 ;D2 22 ;D3 139 ;D4 877 ;D5 6112 ;D6 41874",
            "7k/8/8/1p6/P7/8/8/7K b - - 0 1 ;D1 5 ;D2 22 ;D3 139 ;D4 877 ;D5 6112 ;D6 41874",
            "7k/8/p7/8/8/1P6/8/7K w - - 0 1 ;D1 4 ;D2 16 ;D3 101 ;D4 637 ;D5 4354 ;D6 29679",
            "7k/8/p7/8/8/1P6/8/7K b - - 0 1 ;D1 4 ;D2 16 ;D3 101 ;D4 637 ;D5 4354 ;D6 29679",
            "7k/8/1p6/8/8/P7/8/7K w - - 0 1 ;D1 4 ;D2 16 ;D3 101 ;D4 637 ;D5 4354 ;D6 29679",
            "7k/8/1p6/8/8/P7/8/7K b - - 0 1 ;D1 4 ;D2 16 ;D3 101 ;D4 637 ;D5 4354 ;D6 29679",
            "k7/7p/8/8/8/8/6P1/K7 w - - 0 1 ;D1 5 ;D2 25 ;D3 161 ;D4 1035 ;D5 7574 ;D6 55338",
            "k7/7p/8/8/8/8/6P1/K7 b - - 0 1 ;D1 5 ;D2 25 ;D3 161 ;D4 1035 ;D5 7574 ;D6 55338",
            "k7/6p1/8/8/8/8/7P/K7 w - - 0 1 ;D1 5 ;D2 25 ;D3 161 ;D4 1035 ;D5 7574 ;D6 55338",
            "k7/6p1/8/8/8/8/7P/K7 b - - 0 1 ;D1 5 ;D2 25 ;D3 161 ;D4 1035 ;D5 7574 ;D6 55338",
            "B6b/8/8/8/2K5/4k3/8/b6B w - - 0 1 ;D1 17 ;D2 278 ;D3 4607 ;D4 76778 ;D5 1320507 ;D6 22823890",
            "8/8/1B6/7b/7k/8/2B1b3/7K w - - 0 1 ;D1 21 ;D2 316 ;D3 5744 ;D4 93338 ;D5 1713368 ;D6 28861171",
            "k7/B7/1B6/1B6/8/8/8/K6b w - - 0 1 ;D1 21 ;D2 144 ;D3 3242 ;D4 32955 ;D5 787524 ;D6 7881673",
            "K7/b7/1b6/1b6/8/8/8/k6B w - - 0 1 ;D1 7 ;D2 143 ;D3 1416 ;D4 31787 ;D5 310862 ;D6 7382896",
            "B6b/8/8/8/2K5/5k2/8/b6B b - - 0 1 ;D1 6 ;D2 106 ;D3 1829 ;D4 31151 ;D5 530585 ;D6 9250746",
            "8/8/1B6/7b/7k/8/2B1b3/7K b - - 0 1 ;D1 17 ;D2 309 ;D3 5133 ;D4 93603 ;D5 1591064 ;D6 29027891",
            "k7/B7/1B6/1B6/8/8/8/K6b b - - 0 1 ;D1 7 ;D2 143 ;D3 1416 ;D4 31787 ;D5 310862 ;D6 7382896",
            "K7/b7/1b6/1b6/8/8/8/k6B b - - 0 1 ;D1 21 ;D2 144 ;D3 3242 ;D4 32955 ;D5 787524 ;D6 7881673",
            "7k/RR6/8/8/8/8/rr6/7K w - - 0 1 ;D1 19 ;D2 275 ;D3 5300 ;D4 104342 ;D5 2161211 ;D6 44956585",
            "R6r/8/8/2K5/5k2/8/8/r6R w - - 0 1 ;D1 36 ;D2 1027 ;D3 29215 ;D4 771461 ;D5 20506480 ;D6 525169084",
            "7k/RR6/8/8/8/8/rr6/7K b - - 0 1 ;D1 19 ;D2 275 ;D3 5300 ;D4 104342 ;D5 2161211 ;D6 44956585",
            "R6r/8/8/2K5/5k2/8/8/r6R b - - 0 1 ;D1 36 ;D2 1027 ;D3 29227 ;D4 771368 ;D5 20521342 ;D6 524966748",
            "r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1 ;D1 26 ;D2 568 ;D3 13744 ;D4 314346 ;D5 7594526 ;D6 179862938",
            "r3k2r/8/8/8/8/8/8/1R2K2R b Kkq - 0 1 ;D1 26 ;D2 583 ;D3 14252 ;D4 334705 ;D5 8198901 ;D6 198328929",
            "r3k2r/8/8/8/8/8/8/2R1K2R b Kkq - 0 1 ;D1 25 ;D2 560 ;D3 13592 ;D4 317324 ;D5 7710115 ;D6 185959088",
            "r3k2r/8/8/8/8/8/8/R3K1R1 b Qkq - 0 1 ;D1 25 ;D2 560 ;D3 13607 ;D4 320792 ;D5 7848606 ;D6 190755813",
            "1r2k2r/8/8/8/8/8/8/R3K2R b KQk - 0 1 ;D1 25 ;D2 567 ;D3 14095 ;D4 328965 ;D5 8153719 ;D6 195629489",
            "2r1k2r/8/8/8/8/8/8/R3K2R b KQk - 0 1 ;D1 25 ;D2 548 ;D3 13502 ;D4 312835 ;D5 7736373 ;D6 184411439",
            "r3k1r1/8/8/8/8/8/8/R3K2R b KQq - 0 1 ;D1 25 ;D2 547 ;D3 13579 ;D4 316214 ;D5 7878456 ;D6 189224276",
        ];

        for edp in edps {
            test_edp(edp);
        }
    }
}
