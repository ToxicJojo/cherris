/// Represents a single sqaure on a chess board.
pub struct Square(u8);

impl Square {
    /// The number of sqaures on a chess board.
    pub const COUNT: usize = 64;

    /// Converts a `Square` to a `usize`.
    pub fn to_index(&self) -> usize {
        self.0 as usize
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
}
