use std::fmt::Display;

use crate::{
    bishop_attacks, bishop_xray_attacks, king_attacks, knight_attacks, pawn_attacks, rook_attacks,
    rook_xray_attacks, Bitboard, Color, File, Move, Piece, Rank, Role, Square, RAY_BETWEEN,
};

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
        match chess_move {
            Move::Standard {
                from,
                to,
                role,
                capture,
                ..
            } => {
                let from_bb = Bitboard::from(from);
                let to_bb = Bitboard::from(to);
                let from_to_bb = from_bb ^ to_bb;

                self.color[color] ^= from_to_bb;
                self.role[role] ^= from_to_bb;

                if let Some(capture) = capture {
                    self.color[!color] ^= to_bb;
                    self.role[capture] ^= to_bb;
                }
            }
            Move::EnPassant { from, to, target } => {
                let from_bb = Bitboard::from(from);
                let to_bb = Bitboard::from(to);
                let target_bb = Bitboard::from(target);
                let from_to_bb = from_bb ^ to_bb;

                self.color[color] ^= from_to_bb;
                self.role[Role::Pawn] ^= from_to_bb;

                self.color[!color] ^= target_bb;
                self.role[Role::Pawn] ^= target_bb;
            }
        }
    }

    /// Reverts a move for a color.
    /// This function doesn't check for the legality of the move. If an illegal move is made with
    /// this function it may panic now or cause a panic later.
    pub fn unmake_move(&mut self, color: Color, chess_move: Move) {
        match chess_move {
            Move::Standard {
                from,
                to,
                role,
                capture,
                ..
            } => {
                let from_bb = Bitboard::from(from);
                let to_bb = Bitboard::from(to);
                let from_to_bb = from_bb ^ to_bb;

                self.color[color] ^= from_to_bb;
                self.role[role] ^= from_to_bb;

                if let Some(capture) = capture {
                    self.color[!color] ^= to_bb;
                    self.role[capture] ^= to_bb;
                }
            }
            Move::EnPassant { from, to, target } => {
                let from_bb = Bitboard::from(from);
                let to_bb = Bitboard::from(to);
                let target_bb = Bitboard::from(target);
                let from_to_bb = from_bb ^ to_bb;

                self.color[color] ^= from_to_bb;
                self.role[Role::Pawn] ^= from_to_bb;

                self.color[!color] ^= target_bb;
                self.role[Role::Pawn] ^= target_bb;
            }
        }
    }

    pub fn attacks_on_square(&self, square: Square, color: Color) -> bool {
        let knights = self.role[Role::Knight] & self.color[color];
        let knight_attacks = knight_attacks(square);

        if !(knight_attacks & knights).is_empty() {
            return true;
        }

        let blocker = self.color[Color::White] | self.color[Color::Black];
        let bishops_queens = (self.role[Role::Bishop] | self.role[Role::Queen]) & self.color[color];
        let bishop_queen_attacks = bishop_attacks(square, blocker);
        if !(bishop_queen_attacks & bishops_queens).is_empty() {
            return true;
        }

        let rook_queens = (self.role[Role::Rook] | self.role[Role::Queen]) & self.color[color];
        let rook_queen_attacks = rook_attacks(square, blocker);
        if !(rook_queen_attacks & rook_queens).is_empty() {
            return true;
        }

        let pawns = self.role[Role::Pawn] & self.color[color];
        let pawn_attacks = pawn_attacks(square, !color);
        if !(pawn_attacks & pawns).is_empty() {
            return true;
        }

        false
    }

    pub fn attacked_sqaures(&self, color: Color) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;
        let blocker = (self.color[Color::White] | self.color[Color::Black])
            ^ (self.color[!color] & self.role[Role::King]);

        let kings = self.role[Role::King] & self.color[color];
        for from in kings {
            let king_attacks = king_attacks(from);
            attacks |= king_attacks;
        }

        let knights = self.role[Role::Knight] & self.color[color];
        for from in knights {
            let knight_attacks = knight_attacks(from);
            attacks |= knight_attacks
        }

        let bishops_queens = (self.role[Role::Bishop] | self.role[Role::Queen]) & self.color[color];
        for from in bishops_queens {
            let bishop_queen_attacks = bishop_attacks(from, blocker);
            attacks |= bishop_queen_attacks;
        }

        let rook_queens = (self.role[Role::Rook] | self.role[Role::Queen]) & self.color[color];
        for from in rook_queens {
            let rook_queen_attacks = rook_attacks(from, blocker);
            attacks |= rook_queen_attacks;
        }

        let pawns = self.role[Role::Pawn] & self.color[color];
        for from in pawns {
            let pawn_attacks = pawn_attacks(from, color);
            attacks |= pawn_attacks;
        }

        attacks
    }

    pub fn check_mask(&self, color: Color) -> Bitboard {
        let mut check_mask = Bitboard::EMPTY;
        let kings = self.role[Role::King] & self.color[color];
        let king_sqaure = Square(kings.0.trailing_zeros() as u8);

        let blocker = self.color[Color::White] | self.color[Color::Black];

        let rook_queens = (self.role[Role::Rook] | self.role[Role::Queen]) & self.color[!color];
        let attackers = rook_attacks(king_sqaure, blocker) & rook_queens;
        for attacker in attackers {
            check_mask |= RAY_BETWEEN[king_sqaure.to_index()][attacker.to_index()];
        }

        let bishops_queens =
            (self.role[Role::Bishop] | self.role[Role::Queen]) & self.color[!color];
        let attackers = bishop_attacks(king_sqaure, blocker) & bishops_queens;
        for attacker in attackers {
            check_mask |= RAY_BETWEEN[king_sqaure.to_index()][attacker.to_index()];
        }

        let knights = self.role[Role::Knight] & self.color[!color];
        let attackers = knight_attacks(king_sqaure) & knights;
        for attacker in attackers {
            check_mask |= Bitboard::from(attacker);
        }

        let pawns = self.role[Role::Pawn] & self.color[!color];
        let attackers = pawn_attacks(king_sqaure, color) & pawns;
        for attacker in attackers {
            check_mask |= Bitboard::from(attacker);
        }

        if check_mask.is_empty() {
            Bitboard::FULL
        } else {
            check_mask
        }
    }

    pub fn horizontal_vertical_pinmask(&self, square: Square, color: Color) -> Bitboard {
        let mut pin_mask = Bitboard::EMPTY;
        let blocker = self.color[Color::White] | self.color[Color::Black];
        let rook_queens = (self.role[Role::Rook] | self.role[Role::Queen]) & self.color[color];
        let pinners = rook_xray_attacks(square, blocker) & rook_queens;
        for pinner in pinners {
            pin_mask |= RAY_BETWEEN[square.to_index()][pinner.to_index()];
        }

        pin_mask
    }

    pub fn diagonal_pinmask(&self, square: Square, color: Color) -> Bitboard {
        let mut pin_mask = Bitboard::EMPTY;
        let blocker = self.color[Color::White] | self.color[Color::Black];
        let bishops_queens = (self.role[Role::Bishop] | self.role[Role::Queen]) & self.color[color];
        let pinners = bishop_xray_attacks(square, blocker) & bishops_queens;
        for pinner in pinners {
            pin_mask |= RAY_BETWEEN[square.to_index()][pinner.to_index()];
        }

        pin_mask
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in Rank::iter().rev() {
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
            Move::Standard {
                from: Square::A1,
                to: Square::A8,
                role: Role::Queen,
                capture: None,
                promotion: None,
                en_passant_square: None,
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
            Move::Standard {
                from: Square::A1,
                to: Square::A8,
                role: Role::Queen,
                capture: Some(Role::Queen),
                promotion: None,
                en_passant_square: None,
            },
        );

        assert_eq!(board.piece_on(Square::A1), None);
        assert_eq!(board.piece_on(Square::A8), Some(piece_white));
    }

    #[test]
    fn make_move_en_passant() {
        let mut board = Board::empty();
        let piece_white = Piece {
            color: Color::White,
            role: Role::Pawn,
        };
        let piece_black = Piece {
            color: Color::Black,
            role: Role::Pawn,
        };

        board.put_piece_on(piece_white, Square::D5);
        board.put_piece_on(piece_black, Square::C5);

        board.make_move(
            Color::White,
            Move::EnPassant {
                from: Square::D5,
                to: Square::C6,
                target: Square::C5,
            },
        );

        assert_eq!(board.piece_on(Square::C5), None);
        assert_eq!(board.piece_on(Square::C6), Some(piece_white));
    }

    #[test]
    fn unmake_move_no_capture() {
        let mut board = Board::empty();
        let piece = Piece {
            color: Color::White,
            role: Role::Queen,
        };
        board.put_piece_on(piece, Square::A1);

        let chess_move = Move::Standard {
            from: Square::A1,
            to: Square::A8,
            role: Role::Queen,
            capture: None,
            promotion: None,
            en_passant_square: None,
        };

        board.make_move(Color::White, chess_move);
        board.unmake_move(Color::White, chess_move);

        assert_eq!(board.piece_on(Square::A8), None);
        assert_eq!(board.piece_on(Square::A1), Some(piece));
    }

    #[test]
    fn unmake_move_capture() {
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

        let chess_move = Move::Standard {
            from: Square::A1,
            to: Square::A8,
            role: Role::Queen,
            capture: Some(Role::Queen),
            promotion: None,
            en_passant_square: None,
        };
        board.make_move(Color::White, chess_move);
        board.unmake_move(Color::White, chess_move);

        assert_eq!(board.piece_on(Square::A8), Some(piece_black));
        assert_eq!(board.piece_on(Square::A1), Some(piece_white));
    }

    #[test]
    fn unmake_move_en_passant() {
        let mut board = Board::empty();
        let piece_white = Piece {
            color: Color::White,
            role: Role::Pawn,
        };
        let piece_black = Piece {
            color: Color::Black,
            role: Role::Pawn,
        };

        board.put_piece_on(piece_white, Square::C6);

        board.unmake_move(
            Color::White,
            Move::EnPassant {
                from: Square::D5,
                to: Square::C6,
                target: Square::C5,
            },
        );

        assert_eq!(board.piece_on(Square::D5), Some(piece_white));
        assert_eq!(board.piece_on(Square::C5), Some(piece_black));
    }
}
