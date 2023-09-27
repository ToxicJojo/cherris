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

    pub fn role_on(&self, sqaure: Square) -> Option<Role> {
        let square_bb = Bitboard::from(sqaure);

        for role in Role::iter() {
            if !(self.role[*role] & square_bb).is_empty() {
                return Some(*role);
            }
        }

        None
    }

    pub fn color_on(&self, sqaure: Square) -> Option<Color> {
        let square_bb = Bitboard::from(sqaure);

        if !(self.color[Color::White] & square_bb).is_empty() {
            return Some(Color::White);
        } else if !(self.color[Color::Black] & square_bb).is_empty() {
            return Some(Color::Black);
        }

        None
    }

    pub fn piece_one(&self, sqaure: Square) -> Option<Piece> {
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
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in Rank::ALL.iter().rev() {
            write!(f, "{} ", rank)?;

            for file in File::ALL.iter() {
                let square = Square::from((*file, *rank));
                let piece = self.piece_one(square);

                match piece {
                    Some(piece) => write!(f, "{}", piece)?,
                    None => write!(f, ".")?,
                };

                write!(f, " ")?;
            }
            write!(f, "\n")?;
        }

        write!(f, "\n")?;
        write!(f, "  ")?;

        for file in File::ALL.iter() {
            write!(f, "{} ", file)?;
        }

        Ok(())
    }
}
