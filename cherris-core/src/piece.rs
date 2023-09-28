use std::fmt::Display;

use crate::{Color, Role};

/// Represents a chess piece that has a role and color.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub role: Role,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let role = match self.role {
            Role::Pawn => "p",
            Role::Knight => "n",
            Role::Bishop => "b",
            Role::Rook => "r",
            Role::Queen => "q",
            Role::King => "k",
        };

        match self.color {
            Color::White => write!(f, "{}", role.to_uppercase()),
            Color::Black => write!(f, "{}", role),
        }
    }
}
