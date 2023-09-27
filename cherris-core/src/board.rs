use crate::{Bitboard, Color, Piece};

/// Represents a chess board.
pub struct Board {
    pub pieces: [Bitboard; Piece::COUNT],
    pub colors: [Bitboard; Color::COUNT],
}

impl Board {
    pub fn empty() -> Board {
        Board {
            pieces: [Bitboard::EMPTY; Piece::COUNT],
            colors: [Bitboard::EMPTY; Color::COUNT],
        }
    }
}
