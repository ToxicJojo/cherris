use std::fmt::{Debug, Display};

use crate::{Role, Square};

/// Represents a move in a chess game.
#[derive(Clone, Copy)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub role: Role,
    pub capture: Option<Role>,
    pub promotion: Option<Role>,
    pub en_passant_square: Option<Square>,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.capture.is_some() {
            write!(f, "{}x{}", self.from, self.to)
        } else {
            write!(f, "{}-{}", self.from, self.to)
        }
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.capture.is_some() {
            write!(f, "{}x{}", self.from, self.to)
        } else {
            write!(f, "{}-{}", self.from, self.to)
        }
    }
}
