use std::fmt::Display;

/// Represents a rank on a chess board.
#[derive(Clone, Copy, Debug, PartialEq)]
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

    /// All rank in ascending order.
    pub const ALL: [Rank; Rank::COUNT] = [
        Rank::First,
        Rank::Second,
        Rank::Third,
        Rank::Fourth,
        Rank::Fifth,
        Rank::Sixth,
        Rank::Seventh,
        Rank::Eigth,
    ];

    /// Converts a `Rank` to a `usize`.
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    /// Returns the rank for the given index. Wraps around if index > 7.
    pub fn from_index(index: usize) -> Rank {
        match index % 8 {
            0 => Rank::First,
            1 => Rank::Second,
            2 => Rank::Third,
            3 => Rank::Fourth,
            4 => Rank::Fifth,
            5 => Rank::Sixth,
            6 => Rank::Seventh,
            7 => Rank::Eigth,
            _ => unreachable!(),
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let index = self.to_index() + 1;

        write!(f, "{}", index)
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
    fn const_all() {
        assert_eq!(Rank::ALL.len(), Rank::COUNT);
        assert_eq!(Rank::ALL.first().unwrap(), &Rank::First);
        assert_eq!(Rank::ALL.last().unwrap(), &Rank::Eigth);
    }

    #[test]
    fn to_index() {
        let first = Rank::First.to_index();
        let eight = Rank::Eigth.to_index();

        assert_eq!(first, 0);
        assert_eq!(eight, 7);
    }

    #[test]
    fn from_index() {
        let first = Rank::from_index(0);
        let eigth = Rank::from_index(7);

        assert_eq!(first, Rank::First);
        assert_eq!(eigth, Rank::Eigth)
    }

    #[test]
    fn display() {
        assert_eq!(Rank::First.to_string(), "1");
        assert_eq!(Rank::Eigth.to_string(), "8");
    }
}
