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
