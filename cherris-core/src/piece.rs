use std::fmt::Display;

use crate::{Color, Role};

/// Represents a chess piece that has a role and color.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub role: Role,
}

impl Piece {
    pub const WHITE_KING: Piece = Piece {
        color: Color::White,
        role: Role::King,
    };

    pub const WHITE_QUEEN: Piece = Piece {
        color: Color::White,
        role: Role::Queen,
    };

    pub const WHITE_ROOK: Piece = Piece {
        color: Color::White,
        role: Role::Rook,
    };

    pub const WHITE_KNIGHT: Piece = Piece {
        color: Color::White,
        role: Role::Knight,
    };

    pub const WHITE_BISHOP: Piece = Piece {
        color: Color::White,
        role: Role::Bishop,
    };

    pub const WHITE_PAWN: Piece = Piece {
        color: Color::White,
        role: Role::Pawn,
    };

    pub const BLACK_KING: Piece = Piece {
        color: Color::Black,
        role: Role::King,
    };

    pub const BLACK_QUEEN: Piece = Piece {
        color: Color::Black,
        role: Role::Queen,
    };

    pub const BLACK_ROOK: Piece = Piece {
        color: Color::Black,
        role: Role::Rook,
    };

    pub const BLACK_KNIGHT: Piece = Piece {
        color: Color::Black,
        role: Role::Knight,
    };

    pub const BLACK_BISHOP: Piece = Piece {
        color: Color::Black,
        role: Role::Bishop,
    };

    pub const BLACK_PAWN: Piece = Piece {
        color: Color::Black,
        role: Role::Pawn,
    };
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let role = self.role.to_string();

        match self.color {
            Color::White => write!(f, "{}", role.to_uppercase()),
            Color::Black => write!(f, "{}", role),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(Piece::WHITE_PAWN.to_string(), "P");
        assert_eq!(Piece::WHITE_KNIGHT.to_string(), "N");
        assert_eq!(Piece::WHITE_BISHOP.to_string(), "B");
        assert_eq!(Piece::WHITE_ROOK.to_string(), "R");
        assert_eq!(Piece::WHITE_QUEEN.to_string(), "Q");
        assert_eq!(Piece::WHITE_KING.to_string(), "K");

        assert_eq!(Piece::BLACK_PAWN.to_string(), "p");
        assert_eq!(Piece::BLACK_KNIGHT.to_string(), "n");
        assert_eq!(Piece::BLACK_BISHOP.to_string(), "b");
        assert_eq!(Piece::BLACK_ROOK.to_string(), "r");
        assert_eq!(Piece::BLACK_QUEEN.to_string(), "q");
        assert_eq!(Piece::BLACK_KING.to_string(), "k");
    }
}
