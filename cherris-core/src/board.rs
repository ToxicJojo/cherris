use std::fmt::Display;

use crate::{Bitboard, Color, File, Move, Piece, Rank, Role, Square};

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

    /// Executes a move for a color.
    /// This function doesn't check for the legality of the move. If an illegal move is made with
    /// this function it may panic now or cause a panic later.
    pub fn make_move(&mut self, color: Color, chess_move: Move) {
        let from_bb = Bitboard::from(chess_move.from);
        let to_bb = Bitboard::from(chess_move.to);
        let from_to_bb = from_bb ^ to_bb;

        self.color[color] ^= from_to_bb;
        self.role[chess_move.role] ^= from_to_bb;

        if let Some(capture) = chess_move.capture {
            self.color[!color] ^= to_bb;
            self.role[capture] ^= to_bb;
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in Rank::ALL.iter().rev() {
            write!(f, "{} ", rank)?;

            for file in File::iter() {
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

        for file in File::iter() {
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

    #[test]
    fn make_move_no_capture() {
        let mut board = Board::empty();
        let piece = Piece {
            color: Color::White,
            role: Role::Queen,
        };
        board.put_piece_on(piece, Square::A1);

        board.make_move(
            Color::White,
            Move {
                from: Square::A1,
                to: Square::A8,
                role: Role::Queen,
                capture: None,
                promotion: None,
            },
        );

        assert_eq!(board.piece_on(Square::A1), None);
        assert_eq!(board.piece_on(Square::A8), Some(piece));
    }

    #[test]
    fn make_move_capture() {
        let mut board = Board::empty();
        let piece_white = Piece {
            color: Color::White,
            role: Role::Queen,
        };
        let piece_black = Piece {
            color: Color::Black,
            role: Role::Queen,
        };
        board.put_piece_on(piece_white, Square::A1);
        board.put_piece_on(piece_black, Square::A8);

        board.make_move(
            Color::White,
            Move {
                from: Square::A1,
                to: Square::A8,
                role: Role::Queen,
                capture: Some(Role::Queen),
                promotion: None,
            },
        );

        assert_eq!(board.piece_on(Square::A1), None);
        assert_eq!(board.piece_on(Square::A8), Some(piece_white));
    }
}
