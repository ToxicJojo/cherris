use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    slice::Iter,
    str::FromStr,
};

use crate::Error;

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
            Role::Pawn => write!(f, "P"),
            Role::Knight => write!(f, "N"),
            Role::Bishop => write!(f, "B"),
            Role::Rook => write!(f, "R"),
            Role::Queen => write!(f, "Q"),
            Role::King => write!(f, "K"),
        }
    }
}

impl FromStr for Role {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "p" | "P" => Ok(Role::Pawn),
            "n" | "N" => Ok(Role::Knight),
            "b" | "B" => Ok(Role::Bishop),
            "r" | "R" => Ok(Role::Rook),
            "q" | "Q" => Ok(Role::Queen),
            "k" | "K" => Ok(Role::King),
            _ => Err(Error::ParseRole),
        }
    }
}

impl<T> Index<Role> for [T; Role::COUNT] {
    type Output = T;

    #[inline]
    fn index(&self, index: Role) -> &Self::Output {
        unsafe { self.get_unchecked(index.to_index()) }
    }
}

impl<T> Index<&Role> for [T; Role::COUNT] {
    type Output = T;

    #[inline]
    fn index(&self, index: &Role) -> &Self::Output {
        unsafe { self.get_unchecked(index.to_index()) }
    }
}

impl<T> IndexMut<Role> for [T; Role::COUNT] {
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

    #[test]
    fn to_string() {
        let pawn = Role::Pawn.to_string();
        let knight = Role::Knight.to_string();
        let bishop = Role::Bishop.to_string();
        let rook = Role::Rook.to_string();
        let queen = Role::Queen.to_string();
        let king = Role::King.to_string();

        assert_eq!(pawn, "P");
        assert_eq!(knight, "N");
        assert_eq!(bishop, "B");
        assert_eq!(rook, "R");
        assert_eq!(queen, "Q");
        assert_eq!(king, "K");
    }

    #[test]
    fn from_string() {
        let pawn = Role::from_str("p").unwrap();
        let knight = Role::from_str("n").unwrap();
        let bishop = Role::from_str("b").unwrap();
        let rook = Role::from_str("r").unwrap();
        let queen = Role::from_str("q").unwrap();
        let king = Role::from_str("k").unwrap();

        assert_eq!(pawn, Role::Pawn);
        assert_eq!(knight, Role::Knight);
        assert_eq!(bishop, Role::Bishop);
        assert_eq!(rook, Role::Rook);
        assert_eq!(queen, Role::Queen);
        assert_eq!(king, Role::King);
    }
}
