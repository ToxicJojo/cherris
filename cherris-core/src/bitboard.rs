use std::fmt::Debug;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use crate::{File, Rank, Square};

/// A bitboard where each bit represents a square on a chess board.
#[derive(Clone, Copy)]
pub struct Bitboard(u64);

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);

    pub const fn new(value: u64) -> Bitboard {
        Bitboard(value)
    }

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

impl BitXor for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in Rank::ALL.iter().rev() {
            for file in File::ALL.iter() {
                let square = Square::from((*file, *rank));
                let mask = 1 << square.to_index();

                if self.0 & mask == mask {
                    write!(f, "1 ")?;
                } else {
                    write!(f, "0 ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
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
