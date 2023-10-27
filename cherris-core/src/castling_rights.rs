use std::fmt::Display;

use crate::Color;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CastlingRights {
    NoSide,
    KingSide,
    QueenSide,
    BothSides,
}

impl CastlingRights {
    /// Converts a `&str` to `CastlingRights` for a specific `Color`.
    ///
    /// # Example
    ///
    /// ```
    /// use cherris_core::{CastlingRights, Color};
    ///
    /// let both_sides =  CastlingRights::from_str("KQ", Color::White);
    ///
    /// assert_eq!(both_sides, CastlingRights::BothSides);
    /// ```
    pub fn from_str(input: &str, color: Color) -> CastlingRights {
        let mut castling = CastlingRights::NoSide;
        match color {
            Color::White => {
                if input.contains("KQ") {
                    castling = CastlingRights::BothSides;
                } else if input.contains('K') {
                    castling = CastlingRights::KingSide;
                } else if input.contains('Q') {
                    castling = CastlingRights::QueenSide;
                }
            }
            Color::Black => {
                if input.contains("kq") {
                    castling = CastlingRights::BothSides
                } else if input.contains('k') {
                    castling = CastlingRights::KingSide
                } else if input.contains('q') {
                    castling = CastlingRights::QueenSide
                }
            }
        }
        castling
    }
}

impl Display for CastlingRights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CastlingRights::NoSide => write!(f, ""),
            CastlingRights::KingSide => write!(f, "k"),
            CastlingRights::QueenSide => write!(f, "q"),
            CastlingRights::BothSides => write!(f, "kq"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_white() {
        let no_side = CastlingRights::from_str("", Color::White);
        let king_side = CastlingRights::from_str("K", Color::White);
        let queen_side = CastlingRights::from_str("Q", Color::White);
        let both_sides = CastlingRights::from_str("KQ", Color::White);

        assert_eq!(no_side, CastlingRights::NoSide);
        assert_eq!(king_side, CastlingRights::KingSide);
        assert_eq!(queen_side, CastlingRights::QueenSide);
        assert_eq!(both_sides, CastlingRights::BothSides);
    }

    #[test]
    fn from_str_black() {
        let no_side = CastlingRights::from_str("", Color::Black);
        let king_side = CastlingRights::from_str("k", Color::Black);
        let queen_side = CastlingRights::from_str("q", Color::Black);
        let both_sides = CastlingRights::from_str("kq", Color::Black);

        assert_eq!(no_side, CastlingRights::NoSide);
        assert_eq!(king_side, CastlingRights::KingSide);
        assert_eq!(queen_side, CastlingRights::QueenSide);
        assert_eq!(both_sides, CastlingRights::BothSides);
    }
}
