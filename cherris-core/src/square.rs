use std::str::FromStr;
use std::{fmt::Display, ops::Index, slice::Iter};

use crate::{Bitboard, Error, File, Rank};

/// Represents a single sqaure on a chess board.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square(pub u8);

impl Square {
    /// The number of sqaures on a chess board.
    pub const COUNT: usize = 64;

    #[rustfmt::skip]
    pub const ALL: [Square; Square::COUNT] = [
        Square::A1, Square::B1, Square::C1, Square::D1, Square::E1, Square::F1, Square::G1, Square::H1,
        Square::A2, Square::B2, Square::C2, Square::D2, Square::E2, Square::F2, Square::G2, Square::H2,
        Square::A3, Square::B3, Square::C3, Square::D3, Square::E3, Square::F3, Square::G3, Square::H3,
        Square::A4, Square::B4, Square::C4, Square::D4, Square::E4, Square::F4, Square::G4, Square::H4,
        Square::A5, Square::B5, Square::C5, Square::D5, Square::E5, Square::F5, Square::G5, Square::H5,
        Square::A6, Square::B6, Square::C6, Square::D6, Square::E6, Square::F6, Square::G6, Square::H6,
        Square::A7, Square::B7, Square::C7, Square::D7, Square::E7, Square::F7, Square::G7, Square::H7,
        Square::A8, Square::B8, Square::C8, Square::D8, Square::E8, Square::F8, Square::G8, Square::H8
    ];

    /// Converts a `Square` to a `usize`.
    #[inline]
    pub const fn to_index(&self) -> usize {
        self.0 as usize
    }

    /// Returns an iterator over all sqaures starting from A1 going file by file up to H8.
    #[inline]
    pub fn iter() -> Iter<'static, Square> {
        Square::ALL.iter()
    }

    pub const A1: Square = Square(0);
    pub const B1: Square = Square(1);
    pub const C1: Square = Square(2);
    pub const D1: Square = Square(3);
    pub const E1: Square = Square(4);
    pub const F1: Square = Square(5);
    pub const G1: Square = Square(6);
    pub const H1: Square = Square(7);

    pub const A2: Square = Square(8);
    pub const B2: Square = Square(9);
    pub const C2: Square = Square(10);
    pub const D2: Square = Square(11);
    pub const E2: Square = Square(12);
    pub const F2: Square = Square(13);
    pub const G2: Square = Square(14);
    pub const H2: Square = Square(15);

    pub const A3: Square = Square(16);
    pub const B3: Square = Square(17);
    pub const C3: Square = Square(18);
    pub const D3: Square = Square(19);
    pub const E3: Square = Square(20);
    pub const F3: Square = Square(21);
    pub const G3: Square = Square(22);
    pub const H3: Square = Square(23);

    pub const A4: Square = Square(24);
    pub const B4: Square = Square(25);
    pub const C4: Square = Square(26);
    pub const D4: Square = Square(27);
    pub const E4: Square = Square(28);
    pub const F4: Square = Square(29);
    pub const G4: Square = Square(30);
    pub const H4: Square = Square(31);

    pub const A5: Square = Square(32);
    pub const B5: Square = Square(33);
    pub const C5: Square = Square(34);
    pub const D5: Square = Square(35);
    pub const E5: Square = Square(36);
    pub const F5: Square = Square(37);
    pub const G5: Square = Square(38);
    pub const H5: Square = Square(39);

    pub const A6: Square = Square(40);
    pub const B6: Square = Square(41);
    pub const C6: Square = Square(42);
    pub const D6: Square = Square(43);
    pub const E6: Square = Square(44);
    pub const F6: Square = Square(45);
    pub const G6: Square = Square(46);
    pub const H6: Square = Square(47);

    pub const A7: Square = Square(48);
    pub const B7: Square = Square(49);
    pub const C7: Square = Square(50);
    pub const D7: Square = Square(51);
    pub const E7: Square = Square(52);
    pub const F7: Square = Square(53);
    pub const G7: Square = Square(54);
    pub const H7: Square = Square(55);

    pub const A8: Square = Square(56);
    pub const B8: Square = Square(57);
    pub const C8: Square = Square(58);
    pub const D8: Square = Square(59);
    pub const E8: Square = Square(60);
    pub const F8: Square = Square(61);
    pub const G8: Square = Square(62);
    pub const H8: Square = Square(63);
}

impl From<(File, Rank)> for Square {
    fn from(value: (File, Rank)) -> Self {
        let index = value.0.to_index() + value.1.to_index() * 8;

        Square(index as u8)
    }
}

impl From<&Square> for (File, Rank) {
    fn from(value: &Square) -> Self {
        let file = value.0 % 8;
        let rank = value.0 / 8;

        (File::from_index(file.into()), Rank::from_index(rank.into()))
    }
}

impl Index<Square> for [Bitboard; Square::COUNT] {
    type Output = Bitboard;

    #[inline]
    fn index(&self, index: Square) -> &Self::Output {
        &self[index.to_index()]
    }
}

impl Index<Square> for [u64; Square::COUNT] {
    type Output = u64;

    #[inline]
    fn index(&self, index: Square) -> &Self::Output {
        &self[index.to_index()]
    }
}

impl FromStr for Square {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let file = File::from_str(&s[0..1])?;
        let rank = Rank::from_str(&s[1..2])?;

        Ok(Square::from((file, rank)))
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (file, rank): (File, Rank) = self.into();

        write!(f, "{}{}", file, rank)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn const_count() {
        assert_eq!(Square::COUNT, 64);
    }

    #[test]
    fn to_index() {
        let a1 = Square::A1.to_index();
        let h8 = Square::H8.to_index();

        assert_eq!(a1, 0);
        assert_eq!(h8, 63);
    }

    #[test]
    fn from_file_rank() {
        assert_eq!(Square::from((File::A, Rank::First)), Square(0));
        assert_eq!(Square::from((File::H, Rank::Eigth)), Square(63));
    }

    #[test]
    fn from_str() {
        assert_eq!(Square::from_str("a1").unwrap(), Square::A1);
        assert_eq!(Square::from_str("h8").unwrap(), Square::H8);
    }

    #[test]
    fn display() {
        assert_eq!(Square::A1.to_string(), "a1");
        assert_eq!(Square::H8.to_string(), "h8");
    }
}
