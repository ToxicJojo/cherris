use std::str::FromStr;

use crate::{Color, Move, Position};

pub struct Game {
    position: Position,
}

impl Game {
    pub fn new() -> Game {
        Game {
            position: Position::from_str(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            )
            .unwrap(),
        }
    }

    pub fn color_to_move(&self) -> Color {
        self.position.color_to_move
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        self.position.legal_moves()
    }

    pub fn make_move(&mut self, chess_move: Move) {
        self.position.make_move(chess_move);
    }
}
