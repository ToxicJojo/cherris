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

    pub fn remove_king_side(&mut self) {
        *self = match self {
            CastlingRights::NoSide => CastlingRights::NoSide,
            CastlingRights::KingSide => CastlingRights::NoSide,
            CastlingRights::QueenSide => CastlingRights::QueenSide,
            CastlingRights::BothSides => CastlingRights::QueenSide,
        }
    }

    pub fn remove_queen_side(&mut self) {
        *self = match self {
            CastlingRights::NoSide => CastlingRights::NoSide,
            CastlingRights::KingSide => CastlingRights::KingSide,
            CastlingRights::QueenSide => CastlingRights::NoSide,
            CastlingRights::BothSides => CastlingRights::KingSide,
        }
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

    #[test]
    fn remove_king_side() {
        let mut both = CastlingRights::BothSides;
        let mut king = CastlingRights::KingSide;
        let mut queen = CastlingRights::QueenSide;
        let mut no = CastlingRights::NoSide;

        both.remove_king_side();
        king.remove_king_side();
        queen.remove_king_side();
        no.remove_king_side();

        assert_eq!(both, CastlingRights::QueenSide);
        assert_eq!(king, CastlingRights::NoSide);
        assert_eq!(queen, CastlingRights::QueenSide);
        assert_eq!(no, CastlingRights::NoSide);
    }

    #[test]
    fn remove_queen_side() {
        let mut both = CastlingRights::BothSides;
        let mut king = CastlingRights::KingSide;
        let mut queen = CastlingRights::QueenSide;
        let mut no = CastlingRights::NoSide;

        both.remove_queen_side();
        king.remove_queen_side();
        queen.remove_queen_side();
        no.remove_queen_side();

        assert_eq!(both, CastlingRights::KingSide);
        assert_eq!(king, CastlingRights::KingSide);
        assert_eq!(queen, CastlingRights::NoSide);
        assert_eq!(no, CastlingRights::NoSide);
    }
}
