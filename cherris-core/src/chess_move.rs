use std::fmt::Display;

use crate::{Role, Square};

/// Represents a move in a chess game.
#[derive(Clone, Copy)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub role: Role,
    pub capture: Option<Role>,
    pub promotion: Option<Role>,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(_) = self.capture {
            write!(f, "{}x{}", self.from, self.to)
        } else {
            write!(f, "{}-{}", self.from, self.to)
        }
    }
}
