use crate::{Color, Move};

/// Represents every action that can be taken in a game of chess.
pub enum GameAction {
    MakeMove(Move),
    Resign(Color),
    OfferDraw(Color),
    AcceptDraw,
    DeclareDrawByRepetition,
}
