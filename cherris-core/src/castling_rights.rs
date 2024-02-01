use std::{fmt::Display, str::FromStr};

use crate::Color;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CastlingRights {
    NoSide,
    KingSide,
    QueenSide,
    BothSides,
}

impl CastlingRights {
    /// Converts the `CastlingRights` to a usize.
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    /// Converts a `&str` to `CastlingRights` for a specific `Color`.
    pub fn from_str(input: &str) -> [CastlingRights; Color::COUNT] {
        let mut castling = [CastlingRights::NoSide, CastlingRights::NoSide];
        if input.contains("KQ") {
            castling[Color::White] = CastlingRights::BothSides;
        } else if input.contains('K') {
            castling[Color::White] = CastlingRights::KingSide;
        } else if input.contains('Q') {
            castling[Color::White] = CastlingRights::QueenSide;
        }
        if input.contains("kq") {
            castling[Color::Black] = CastlingRights::BothSides
        } else if input.contains('k') {
            castling[Color::Black] = CastlingRights::KingSide
        } else if input.contains('q') {
            castling[Color::Black] = CastlingRights::QueenSide
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
        let no_side = CastlingRights::from_str("");
        let king_side = CastlingRights::from_str("K");
        let queen_side = CastlingRights::from_str("Q");
        let both_sides = CastlingRights::from_str("KQ");

        assert_eq!(no_side, [CastlingRights::NoSide, CastlingRights::NoSide]);
        assert_eq!(
            king_side,
            [CastlingRights::KingSide, CastlingRights::NoSide]
        );
        assert_eq!(
            queen_side,
            [CastlingRights::QueenSide, CastlingRights::NoSide]
        );
        assert_eq!(
            both_sides,
            [CastlingRights::BothSides, CastlingRights::NoSide]
        );
    }

    #[test]
    fn from_str_black() {
        let no_side = CastlingRights::from_str("");
        let king_side = CastlingRights::from_str("k");
        let queen_side = CastlingRights::from_str("q");
        let both_sides = CastlingRights::from_str("kq");

        assert_eq!(no_side, [CastlingRights::NoSide, CastlingRights::NoSide]);
        assert_eq!(
            king_side,
            [CastlingRights::NoSide, CastlingRights::KingSide]
        );
        assert_eq!(
            queen_side,
            [CastlingRights::NoSide, CastlingRights::QueenSide]
        );
        assert_eq!(
            both_sides,
            [CastlingRights::NoSide, CastlingRights::BothSides,]
        );
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
