use crate::{Role, Square};

/// Represents a move in a chess game.
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub role: Role,
    pub capture: Option<Role>,
    pub promotion: Option<Role>,
}
