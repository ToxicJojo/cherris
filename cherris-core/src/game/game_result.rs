use crate::Color;

/// Represents the result of a chess game.
#[derive(Clone, Copy)]
pub enum GameResult {
    Win(Color),
    Draw,
    Ongoing,
}
