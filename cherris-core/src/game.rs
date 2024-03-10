use crate::{Color, Move, MoveList, Position};

use self::game_result::GameResult;

mod game_action;
mod game_result;

pub struct Game {
    position: Position,
    game_result: GameResult,
}

impl Game {
    pub fn new() -> Game {
        Game {
            position: Position::default(),
            game_result: GameResult::Ongoing,
        }
    }

    pub fn color_to_move(&self) -> Color {
        self.position.color_to_move
    }

    pub fn moves(&self) -> MoveList {
        self.position.legal_moves()
    }

    pub fn make_move(&mut self, chess_move: Move) {
        if self.game_result == GameResult::Ongoing {
            self.position.make_move(chess_move);

            if self.position().is_checkmate() {
                self.game_result = GameResult::Win(!self.position.color_to_move);
            } else if self.position().is_stalemate() {
                self.game_result = GameResult::Draw;
            }
        }
    }

    pub fn result(&self) -> GameResult {
        self.game_result
    }

    pub fn position(&self) -> &Position {
        &self.position
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
