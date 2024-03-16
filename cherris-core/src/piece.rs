use std::fmt::Display;

use crate::{Color, Error, Role};

/// Represents a chess piece that has a role and color.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub role: Role,
}

impl Piece {
    pub const WHITE_KING: Piece = Piece {
        color: Color::White,
        role: Role::King,
    };

    pub const WHITE_QUEEN: Piece = Piece {
        color: Color::White,
        role: Role::Queen,
    };

    pub const WHITE_ROOK: Piece = Piece {
        color: Color::White,
        role: Role::Rook,
    };

    pub const WHITE_KNIGHT: Piece = Piece {
        color: Color::White,
        role: Role::Knight,
    };

    pub const WHITE_BISHOP: Piece = Piece {
        color: Color::White,
        role: Role::Bishop,
    };

    pub const WHITE_PAWN: Piece = Piece {
        color: Color::White,
        role: Role::Pawn,
    };

    pub const BLACK_KING: Piece = Piece {
        color: Color::Black,
        role: Role::King,
    };

    pub const BLACK_QUEEN: Piece = Piece {
        color: Color::Black,
        role: Role::Queen,
    };

    pub const BLACK_ROOK: Piece = Piece {
        color: Color::Black,
        role: Role::Rook,
    };

    pub const BLACK_KNIGHT: Piece = Piece {
        color: Color::Black,
        role: Role::Knight,
    };

    pub const BLACK_BISHOP: Piece = Piece {
        color: Color::Black,
        role: Role::Bishop,
    };

    pub const BLACK_PAWN: Piece = Piece {
        color: Color::Black,
        role: Role::Pawn,
    };
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let role = self.role.to_string();

        match self.color {
            Color::White => write!(f, "{}", role),
            Color::Black => write!(f, "{}", role.to_lowercase()),
        }
    }
}

impl TryFrom<char> for Piece {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'P' => Ok(Piece::WHITE_PAWN),
            'p' => Ok(Piece::BLACK_PAWN),
            'N' => Ok(Piece::WHITE_KNIGHT),
            'n' => Ok(Piece::BLACK_KNIGHT),
            'B' => Ok(Piece::WHITE_BISHOP),
            'b' => Ok(Piece::BLACK_BISHOP),
            'R' => Ok(Piece::WHITE_ROOK),
            'r' => Ok(Piece::BLACK_ROOK),
            'Q' => Ok(Piece::WHITE_QUEEN),
            'q' => Ok(Piece::BLACK_QUEEN),
            'K' => Ok(Piece::WHITE_KING),
            'k' => Ok(Piece::BLACK_KING),
            _ => Err(Error::ParsePiece),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(Piece::WHITE_PAWN.to_string(), "P");
        assert_eq!(Piece::WHITE_KNIGHT.to_string(), "N");
        assert_eq!(Piece::WHITE_BISHOP.to_string(), "B");
        assert_eq!(Piece::WHITE_ROOK.to_string(), "R");
        assert_eq!(Piece::WHITE_QUEEN.to_string(), "Q");
        assert_eq!(Piece::WHITE_KING.to_string(), "K");

        assert_eq!(Piece::BLACK_PAWN.to_string(), "p");
        assert_eq!(Piece::BLACK_KNIGHT.to_string(), "n");
        assert_eq!(Piece::BLACK_BISHOP.to_string(), "b");
        assert_eq!(Piece::BLACK_ROOK.to_string(), "r");
        assert_eq!(Piece::BLACK_QUEEN.to_string(), "q");
        assert_eq!(Piece::BLACK_KING.to_string(), "k");
    }

    #[test]
    fn try_from_char() {
        assert_eq!(Piece::try_from('P'), Ok(Piece::WHITE_PAWN));
        assert_eq!(Piece::try_from('N'), Ok(Piece::WHITE_KNIGHT));
        assert_eq!(Piece::try_from('B'), Ok(Piece::WHITE_BISHOP));
        assert_eq!(Piece::try_from('R'), Ok(Piece::WHITE_ROOK));
        assert_eq!(Piece::try_from('Q'), Ok(Piece::WHITE_QUEEN));
        assert_eq!(Piece::try_from('K'), Ok(Piece::WHITE_KING));

        assert_eq!(Piece::try_from('p'), Ok(Piece::BLACK_PAWN));
        assert_eq!(Piece::try_from('n'), Ok(Piece::BLACK_KNIGHT));
        assert_eq!(Piece::try_from('b'), Ok(Piece::BLACK_BISHOP));
        assert_eq!(Piece::try_from('r'), Ok(Piece::BLACK_ROOK));
        assert_eq!(Piece::try_from('q'), Ok(Piece::BLACK_QUEEN));
        assert_eq!(Piece::try_from('k'), Ok(Piece::BLACK_KING));

        assert_eq!(Piece::try_from('X'), Err(Error::ParsePiece))
    }
}
