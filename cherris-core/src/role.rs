use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    slice::Iter,
};

use crate::Bitboard;

/// Represents the role of a chess piece.
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
    #[inline]
    pub fn iter() -> Iter<'static, Role> {
        Role::ALL.iter()
    }

    /// Converts a `Role` to a `usize`.
    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Pawn => write!(f, "p"),
            Role::Knight => write!(f, "n"),
            Role::Bishop => write!(f, "b"),
            Role::Rook => write!(f, "r"),
            Role::Queen => write!(f, "q"),
            Role::King => write!(f, "k"),
        }
    }
}

impl Index<Role> for [Bitboard; Role::COUNT] {
    type Output = Bitboard;

    #[inline]
    fn index(&self, index: Role) -> &Self::Output {
        unsafe { self.get_unchecked(index.to_index()) }
    }
}

impl IndexMut<Role> for [Bitboard; Role::COUNT] {
    #[inline]
    fn index_mut(&mut self, index: Role) -> &mut Self::Output {
        unsafe { self.get_unchecked_mut(index.to_index()) }
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
