use std::{
    ops::{Index, IndexMut, Not},
    slice::Iter,
    str::FromStr,
};

use crate::{Bitboard, Error};

/// Represents the colors in chess.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    /// The number of colors in chess.
    pub const COUNT: usize = 2;

    /// All colors in chess starting with white.
    pub const ALL: [Color; Color::COUNT] = [Color::White, Color::Black];

    /// An iterator over all colors starting with white.
    pub fn iter() -> Iter<'static, Color> {
        Color::ALL.iter()
    }

    /// Converts the `Color` to a usize.
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

impl Index<Color> for [Bitboard; Color::COUNT] {
    type Output = Bitboard;

    fn index(&self, index: Color) -> &Self::Output {
        &self[index.to_index()]
    }
}

impl IndexMut<Color> for [Bitboard; Color::COUNT] {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        &mut self[index.to_index()]
    }
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl FromStr for Color {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Color::White),
            "b" => Ok(Color::Black),
            _ => Err(Error::ParseColor),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn const_count() {
        assert_eq!(Color::COUNT, 2);
    }

    #[test]
    fn const_all() {
        assert_eq!(Color::ALL.len(), Color::COUNT);
        assert_eq!(Color::ALL.first().unwrap(), &Color::White);
        assert_eq!(Color::ALL.last().unwrap(), &Color::Black);
    }

    #[test]
    fn to_index() {
        let white = Color::White;
        let black = Color::Black;

        assert_eq!(white.to_index(), 0);
        assert_eq!(black.to_index(), 1);
    }

    #[test]
    fn from_str() {
        assert_eq!(Color::from_str("w").unwrap(), Color::White);
        assert_eq!(Color::from_str("b").unwrap(), Color::Black);
    }

    #[test]
    fn from_str_error() {
        assert_eq!(Color::from_str("c"), Err(Error::ParseColor));
    }
}
