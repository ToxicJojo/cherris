/// Represents the colors in chess.
#[derive(Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Color {
    /// The number of colors in chess.
    pub const COUNT: usize = 2;

    /// Converts the `Color` to a usize.
    pub fn to_index(&self) -> usize {
        *self as usize
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
    fn to_index_white() {
        let white = Color::White;

        assert_eq!(white.to_index(), 0);
    }

    #[test]
    fn to_index_black() {
        let black = Color::Black;

        assert_eq!(black.to_index(), 1);
    }
}
