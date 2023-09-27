use crate::{File, Rank};

/// Represents a single sqaure on a chess board.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square(u8);

impl Square {
    /// The number of sqaures on a chess board.
    pub const COUNT: usize = 64;

    /// Converts a `Square` to a `usize`.
    pub fn to_index(&self) -> usize {
        self.0 as usize
    }
}

impl From<(File, Rank)> for Square {
    fn from(value: (File, Rank)) -> Self {
        let index = value.0.to_index() + value.1.to_index() * 8;

        Square(index as u8)
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
        let a1 = Square(0).to_index();
        let h8 = Square(63).to_index();

        assert_eq!(a1, 0);
        assert_eq!(h8, 63);
    }

    #[test]
    fn from_file_rank() {
        assert_eq!(Square::from((File::A, Rank::First)), Square(0));
        assert_eq!(Square::from((File::H, Rank::Eigth)), Square(63));
    }
}
