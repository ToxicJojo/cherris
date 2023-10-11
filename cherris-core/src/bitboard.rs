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

impl Iterator for Bitboard {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let index = self.0.trailing_zeros();
            let sqaure = Square(index as u8);
            self.0 &= !(1 << index);

            Some(sqaure)
        }
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
        for rank in Rank::iter().rev() {
            for file in File::iter() {
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

    #[test]
    fn iterator() {
        let bitboard = Bitboard::from(Square::A1) | Bitboard::from(Square::H8);

        let mut iter = bitboard.into_iter();
        assert_eq!(iter.next(), Some(Square::A1));
        assert_eq!(iter.next(), Some(Square::H8));
        assert_eq!(iter.next(), None);
    }
}
