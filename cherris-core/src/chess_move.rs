use std::fmt::{Debug, Display};

use crate::{Role, Square};

/// Represents a move in a chess game.
#[derive(Clone, Copy)]
pub enum Move {
    Standard {
        from: Square,
        to: Square,
        role: Role,
        capture: Option<Role>,
        promotion: Option<Role>,
        en_passant_square: Option<Square>,
    },
    EnPassant {
        from: Square,
        to: Square,
        target: Square,
    },
    CastleShort,
    CastleLong,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Standard {
                from,
                to,
                capture,
                promotion,
                ..
            } => {
                if capture.is_some() {
                    if let Some(promotion) = promotion {
                        write!(f, "{}x{}={}", from, to, promotion)
                    } else {
                        write!(f, "{}x{}", from, to)
                    }
                } else if let Some(promotion) = promotion {
                    write!(f, "{}-{}={}", from, to, promotion)
                } else {
                    write!(f, "{}-{}", from, to)
                }
            }
            Move::EnPassant { from, to, .. } => {
                write!(f, "{}x{}", from, to)
            }
            Move::CastleShort => write!(f, "O-O"),
            Move::CastleLong => write!(f, "O-O-O"),
        }
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Standard {
                from, to, capture, ..
            } => {
                if capture.is_some() {
                    write!(f, "{}x{}", from, to)
                } else {
                    write!(f, "{}-{}", from, to)
                }
            }
            Move::EnPassant { from, to, .. } => {
                write!(f, "{}x{}", from, to)
            }
            Move::CastleShort => write!(f, "O-O"),
            Move::CastleLong => write!(f, "O-O-O"),
        }
    }
}
