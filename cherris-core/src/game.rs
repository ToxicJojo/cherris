use std::str::FromStr;

use arrayvec::ArrayVec;

use crate::{Color, GameAction, Move, Position};

pub struct Game {
    position: Position,
    game_actions: Vec<GameAction>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            position: Position::from_str(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            )
            .unwrap(),
            game_actions: Vec::new(),
        }
    }

    pub fn color_to_move(&self) -> Color {
        self.position.color_to_move
    }

    pub fn game_actions(&self) -> &Vec<GameAction> {
        &self.game_actions
    }

    pub fn legal_moves(&self) -> ArrayVec<Move, 256> {
        self.position.legal_moves()
    }

    pub fn make_move(&mut self, chess_move: Move) {
        self.game_actions.push(GameAction::MakeMove(chess_move));
        self.position.make_move(chess_move);
    }

    pub fn resign(&mut self, color: Color) {
        self.game_actions.push(GameAction::Resign(color));
    }

    pub fn offer_draw(&mut self, color: Color) {
        self.game_actions.push(GameAction::OfferDraw(color));
    }

    pub fn accept_draw(&mut self) {
        self.game_actions.push(GameAction::AcceptDraw);
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
