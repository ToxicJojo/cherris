use std::fmt::Display;

use crate::Color;

/// Represents the result of a chess game.
#[derive(Clone, Copy, PartialEq)]
pub enum GameResult {
    Win(Color),
    Draw,
    Ongoing,
}

impl Display for GameResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameResult::Win(Color::White) => write!(f, "1-0"),
            GameResult::Win(Color::Black) => write!(f, "0-1"),
            GameResult::Draw => write!(f, "½-½"),
            GameResult::Ongoing => write!(f, "0-0"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_win_white() {
        let result = GameResult::Win(Color::White).to_string();
        assert_eq!(result, "1-0");
    }

    #[test]
    fn display_win_black() {
        let result = GameResult::Win(Color::Black).to_string();
        assert_eq!(result, "0-1");
    }

    #[test]
    fn display_win_draw() {
        let result = GameResult::Draw.to_string();
        assert_eq!(result, "½-½");
    }

    #[test]
    fn display_win_ongoing() {
        let result = GameResult::Ongoing.to_string();
        assert_eq!(result, "0-0");
    }
}
