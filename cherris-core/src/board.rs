use std::fmt::Display;

use crate::{Bitboard, Color, File, Piece, Rank, Role, Square};

/// Represents a chess board.
pub struct Board {
    pub role: [Bitboard; Role::COUNT],
    pub color: [Bitboard; Color::COUNT],
}

impl Board {
    pub fn empty() -> Board {
        Board {
            role: [Bitboard::EMPTY; Role::COUNT],
            color: [Bitboard::EMPTY; Color::COUNT],
        }
    }

    /// Return the `Role` on a given `Sqaure`.
    pub fn role_on(&self, sqaure: Square) -> Option<Role> {
        let square_bb = Bitboard::from(sqaure);

        for role in Role::iter() {
            if !(self.role[*role] & square_bb).is_empty() {
                return Some(*role);
            }
        }

        None
    }

    /// Return the `Color` on a given `Sqaure`.
    pub fn color_on(&self, sqaure: Square) -> Option<Color> {
        let square_bb = Bitboard::from(sqaure);

        if !(self.color[Color::White] & square_bb).is_empty() {
            return Some(Color::White);
        } else if !(self.color[Color::Black] & square_bb).is_empty() {
            return Some(Color::Black);
        }

        None
    }

    /// Return the `Piece` on a given `Sqaure`.
    pub fn piece_on(&self, sqaure: Square) -> Option<Piece> {
        let color = self.color_on(sqaure);
        let role = self.role_on(sqaure);

        match (color, role) {
            (Some(color), Some(role)) => Some(Piece { color, role }),
            (None, None) => None,
            (_, _) => {
                panic!("Invalid board state encountered. The color and role bitboards don't match",)
            }
        }
    }

    /// Puts the given `Piece` on a `Square`.
    pub fn put_piece_on(&mut self, piece: Piece, sqaure: Square) {
        let square_bb = Bitboard::from(sqaure);

        self.color[piece.color] |= square_bb;
        self.color[!piece.color] &= !square_bb;

        self.role[piece.role] |= square_bb;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in Rank::ALL.iter().rev() {
            write!(f, "{} ", rank)?;

            for file in File::ALL.iter() {
                let square = Square::from((*file, *rank));
                let piece = self.piece_on(square);

                match piece {
                    Some(piece) => write!(f, "{}", piece)?,
                    None => write!(f, ".")?,
                };

                write!(f, " ")?;
            }
            writeln!(f)?;
        }

        writeln!(f)?;
        write!(f, "  ")?;

        for file in File::ALL.iter() {
            write!(f, "{} ", file)?;
        }

        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn piece_on() {
        let mut board = Board::empty();
        let piece = Piece {
            color: Color::White,
            role: Role::Pawn,
        };
        let square = Square::D5;
        board.put_piece_on(piece, square);

        assert_eq!(board.piece_on(square).unwrap(), piece);
    }

    #[test]
    fn color_on() {
        let mut board = Board::empty();
        let piece = Piece {
            color: Color::Black,
            role: Role::Pawn,
        };
        let square = Square::C4;
        board.put_piece_on(piece, square);

        assert_eq!(board.color_on(square).unwrap(), piece.color);
    }

    #[test]
    fn role_on() {
        let mut board = Board::empty();
        let piece = Piece {
            color: Color::Black,
            role: Role::Bishop,
        };
        let square = Square::H8;
        board.put_piece_on(piece, square);

        assert_eq!(board.role_on(square).unwrap(), piece.role);
    }
}
