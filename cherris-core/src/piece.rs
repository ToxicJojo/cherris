/// Represents the type of a chess piece.
#[derive(Clone, Copy)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    /// The number of different chess pieces.
    pub const COUNT: usize = 6;

    /// Converts a `Piece` to a `usize`.
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn const_count() {
        assert_eq!(Piece::COUNT, 6);
    }

    #[test]
    fn to_index() {
        let pawn = Piece::Pawn.to_index();
        let knight = Piece::Knight.to_index();
        let bishop = Piece::Bishop.to_index();
        let rook = Piece::Rook.to_index();
        let queen = Piece::Queen.to_index();
        let king = Piece::King.to_index();

        assert_eq!(pawn, 0);
        assert_eq!(knight, 1);
        assert_eq!(bishop, 2);
        assert_eq!(rook, 3);
        assert_eq!(queen, 4);
        assert_eq!(king, 5);
    }
}
