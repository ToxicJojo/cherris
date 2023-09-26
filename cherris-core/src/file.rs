/// Represents a file on a chess board.
#[derive(Clone, Copy)]
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

    /// Converts a `File` to a `usize`.
    pub fn to_index(&self) -> usize {
        *self as usize
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
    fn to_index() {
        let a = File::A.to_index();
        let h = File::H.to_index();

        assert_eq!(a, 0);
        assert_eq!(h, 7);
    }
}
