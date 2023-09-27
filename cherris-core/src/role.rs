use std::{ops::Index, slice::Iter};

use crate::Bitboard;

/// Represents the type of a chess piece.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Role {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Role {
    /// The number of different chess pieces.
    pub const COUNT: usize = 6;

    /// All roles in chess.
    pub const ALL: [Role; Role::COUNT] = [
        Role::Pawn,
        Role::Knight,
        Role::Bishop,
        Role::Rook,
        Role::Queen,
        Role::King,
    ];

    /// Returns an iterator over all roles.
    pub fn iter() -> Iter<'static, Role> {
        Role::ALL.iter()
    }

    /// Converts a `Piece` to a `usize`.
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

impl Index<Role> for [Bitboard; Role::COUNT] {
    type Output = Bitboard;

    fn index(&self, index: Role) -> &Self::Output {
        &self[index.to_index()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn const_count() {
        assert_eq!(Role::COUNT, 6);
    }

    #[test]
    fn const_all() {
        assert_eq!(Role::ALL.len(), Role::COUNT);
        assert_eq!(*Role::ALL.first().unwrap(), Role::Pawn);
        assert_eq!(*Role::ALL.last().unwrap(), Role::King);
    }

    #[test]
    fn to_index() {
        let pawn = Role::Pawn.to_index();
        let knight = Role::Knight.to_index();
        let bishop = Role::Bishop.to_index();
        let rook = Role::Rook.to_index();
        let queen = Role::Queen.to_index();
        let king = Role::King.to_index();

        assert_eq!(pawn, 0);
        assert_eq!(knight, 1);
        assert_eq!(bishop, 2);
        assert_eq!(rook, 3);
        assert_eq!(queen, 4);
        assert_eq!(king, 5);
    }
}
