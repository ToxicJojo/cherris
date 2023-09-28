use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

use crate::Square;

/// A bitboard where each bit represents a square on a chess board.
#[derive(Clone, Copy)]
pub struct Bitboard(u64);

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);

    /// Determines whether the bitboard is empty or not.
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl From<Square> for Bitboard {
    fn from(value: Square) -> Self {
        Bitboard(1 << value.to_index())
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty() {
        let bitboard = Bitboard::EMPTY;

        assert!(bitboard.is_empty());
    }

    #[test]
    fn is_not_empty() {
        let bitboard = Bitboard(1);

        assert!(!bitboard.is_empty());
    }
}
