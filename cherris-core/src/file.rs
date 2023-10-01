use std::{fmt::Display, str::FromStr};

use crate::Error;

/// Represents a file on a chess board.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    /// The number of files on a chess board.
    pub const COUNT: usize = 8;

    /// All files in ascending order.
    pub const ALL: [File; File::COUNT] = [
        File::A,
        File::B,
        File::C,
        File::D,
        File::E,
        File::F,
        File::G,
        File::H,
    ];

    /// Converts a `File` to a `usize`.
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    /// Returns the file for the given index. Wraps around if index > 7.
    pub fn from_index(index: usize) -> File {
        match index % 8 {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => unreachable!(),
        }
    }

    /// Returns one file to the left. Wraps around.
    pub fn left(&self) -> File {
        File::from_index(self.to_index().wrapping_sub(1))
    }

    /// Returns one file to the right. Wraps around.
    pub fn right(&self) -> File {
        File::from_index(self.to_index() + 1)
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            File::A => write!(f, "a"),
            File::B => write!(f, "b"),
            File::C => write!(f, "c"),
            File::D => write!(f, "d"),
            File::E => write!(f, "e"),
            File::F => write!(f, "f"),
            File::G => write!(f, "g"),
            File::H => write!(f, "h"),
        }
    }
}

impl FromStr for File {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(File::A),
            "b" => Ok(File::B),
            "c" => Ok(File::C),
            "d" => Ok(File::D),
            "e" => Ok(File::E),
            "f" => Ok(File::F),
            "g" => Ok(File::G),
            "h" => Ok(File::H),
            _ => Err(Error::ParseFile),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn const_count() {
        assert_eq!(File::COUNT, 8);
    }

    #[test]
    fn const_all() {
        assert_eq!(File::ALL.len(), File::COUNT);
        assert_eq!(File::ALL.first().unwrap(), &File::A);
        assert_eq!(File::ALL.last().unwrap(), &File::H);
    }

    #[test]
    fn to_index() {
        let a = File::A.to_index();
        let h = File::H.to_index();

        assert_eq!(a, 0);
        assert_eq!(h, 7);
    }

    #[test]
    fn from_index() {
        let a = File::from_index(0);
        let h = File::from_index(7);

        assert_eq!(a, File::A);
        assert_eq!(h, File::H);
    }

    #[test]
    fn left() {
        assert_eq!(File::A.left(), File::H);
        assert_eq!(File::H.left(), File::G);
    }

    #[test]
    fn right() {
        assert_eq!(File::A.right(), File::B);
        assert_eq!(File::H.right(), File::A);
    }

    #[test]
    fn from_str() {
        assert_eq!(File::from_str("a").unwrap(), File::A);
        assert_eq!(File::from_str("h").unwrap(), File::H);
    }

    #[test]
    fn display() {
        assert_eq!(File::A.to_string(), "a");
        assert_eq!(File::H.to_string(), "h");
    }
}
