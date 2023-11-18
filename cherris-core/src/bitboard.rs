use std::fmt::Debug;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

use crate::{Color, File, Rank, Square};

/// A bitboard where each bit represents a square on a chess board.
#[derive(Clone, Copy, PartialEq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const FULL: Bitboard = Bitboard(u64::MAX);

    pub const FIRST_RANK: Bitboard = Bitboard(0x00000000000000FF);
    pub const SECOND_RANK: Bitboard = Bitboard(0x000000000000FF00);
    pub const THIRD_RANK: Bitboard = Bitboard(0x0000000000FF0000);
    pub const SEVENTH_RANK: Bitboard = Bitboard(0x00FF000000000000);
    pub const EIGTH_RANK: Bitboard = Bitboard(0xFF00000000000000);

    pub const PROMOTION_RANK: [Bitboard; Color::COUNT] =
        [Bitboard::EIGTH_RANK, Bitboard::FIRST_RANK];

    pub const fn new(value: u64) -> Bitboard {
        Bitboard(value)
    }

    /// Determines whether the bitboard is empty or not.
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Determines the total amount of occupied squares on this bitboard.
    pub fn population_count(&self) -> u32 {
        self.0.count_ones()
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

impl From<&[Square]> for Bitboard {
    fn from(value: &[Square]) -> Self {
        let mut bb = Bitboard::EMPTY;
        for square in value {
            bb |= Bitboard::from(*square);
        }

        bb
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

impl Shl<u64> for Bitboard {
    type Output = Bitboard;

    fn shl(self, rhs: u64) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl Shr<u64> for Bitboard {
    type Output = Bitboard;

    fn shr(self, rhs: u64) -> Self::Output {
        Bitboard(self.0 >> rhs)
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

    #[test]
    fn population_count_empty() {
        let pop_count = Bitboard::EMPTY.population_count();

        assert_eq!(pop_count, 0);
    }

    #[test]
    fn population_count_full() {
        let pop_count = Bitboard::FULL.population_count();

        assert_eq!(pop_count, 64);
    }
}
