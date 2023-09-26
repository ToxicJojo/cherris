/// Represents a rank on a chess board.
#[derive(Clone, Copy)]
pub enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eigth,
}

impl Rank {
    /// The number of ranks on a chess board.
    pub const COUNT: usize = 8;

    /// Converts a `Rank` to a `usize`.
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn const_count() {
        assert_eq!(Rank::COUNT, 8);
    }

    #[test]
    fn to_index() {
        let first = Rank::First.to_index();
        let eight = Rank::Eigth.to_index();

        assert_eq!(first, 0);
        assert_eq!(eight, 7);
    }
}
