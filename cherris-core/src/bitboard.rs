/// A bitboard where each bit represents a square on a chess board.
pub struct Bitboard(u64);

impl Bitboard {
    /// Determines whether the bitboard is empty or not.
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty() {
        let bitboard = Bitboard(0);

        assert_eq!(bitboard.is_empty(), true);
    }

    #[test]
    fn is_not_empty() {
        let bitboard = Bitboard(1);

        assert_eq!(bitboard.is_empty(), false);
    }
}
