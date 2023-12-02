use std::fmt::Display;

use crate::{
    bishop_attacks, bishop_xray_attacks, king_attacks, knight_attacks, pawn_attacks, rook_attacks,
    rook_xray_attacks, Bitboard, Color, File, Move, Piece, Rank, Role, Square, RAY_BETWEEN,
};

/// Represents a chess board.
pub struct Board {
    pub role: [Bitboard; Role::COUNT],
    pub color: [Bitboard; Color::COUNT],
    pub occupied: Bitboard,
}

impl Board {
    /// An empty board with no pieces on it.
    pub const EMPTY: Board = Board {
        role: [Bitboard::EMPTY; Role::COUNT],
        color: [Bitboard::EMPTY; Color::COUNT],
        occupied: Bitboard::EMPTY,
    };

    /// Return the `Role` on a given `Sqaure`.
    #[inline]
    pub fn role_on(&self, sqaure: Square) -> Option<Role> {
        let square_bb = Bitboard::from(sqaure);

        if (self.occupied & square_bb).is_empty() {
            None
        } else if !(self.role[Role::Pawn] & square_bb).is_empty() {
            Some(Role::Pawn)
        } else if !(self.role[Role::Knight] & square_bb).is_empty() {
            Some(Role::Knight)
        } else if !(self.role[Role::Bishop] & square_bb).is_empty() {
            Some(Role::Bishop)
        } else if !(self.role[Role::Rook] & square_bb).is_empty() {
            Some(Role::Rook)
        } else if !(self.role[Role::Queen] & square_bb).is_empty() {
            Some(Role::Queen)
        } else {
            Some(Role::King)
        }
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

        if let Some(role) = self.role_on(sqaure) {
            self.role[role] &= !square_bb;
        }

        self.role[piece.role] |= square_bb;
        self.occupied |= square_bb;
    }

    pub fn count_roles(&self, role: Role, color: Color) -> u32 {
        (self.role[role] & self.color[color]).population_count()
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
                promotion,
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

                if let Some(promotion) = promotion {
                    self.role[role] ^= to_bb;
                    self.role[promotion] ^= to_bb;
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
            Move::CastleShort => match color {
                Color::White => {
                    let king_bb = Bitboard(1 << 4) | Bitboard(1 << 6);
                    let rook_bb = Bitboard(1 << 5) | Bitboard(1 << 7);

                    self.color[color] ^= king_bb | rook_bb;
                    self.role[Role::King] ^= king_bb;
                    self.role[Role::Rook] ^= rook_bb;
                }
                Color::Black => {
                    let king_bb = Bitboard(1 << 60) | Bitboard(1 << 62);
                    let rook_bb = Bitboard(1 << 61) | Bitboard(1 << 63);

                    self.color[color] ^= king_bb | rook_bb;
                    self.role[Role::King] ^= king_bb;
                    self.role[Role::Rook] ^= rook_bb;
                }
            },
            Move::CastleLong => match color {
                Color::White => {
                    let king_bb = Bitboard(1 << 4) | Bitboard(1 << 2);
                    let rook_bb = Bitboard(1 << 0) | Bitboard(1 << 3);

                    self.color[color] ^= king_bb | rook_bb;
                    self.role[Role::King] ^= king_bb;
                    self.role[Role::Rook] ^= rook_bb;
                }
                Color::Black => {
                    let king_bb = Bitboard(1 << 60) | Bitboard(1 << 58);
                    let rook_bb = Bitboard(1 << 56) | Bitboard(1 << 59);

                    self.color[color] ^= king_bb | rook_bb;
                    self.role[Role::King] ^= king_bb;
                    self.role[Role::Rook] ^= rook_bb;
                }
            },
        }

        self.occupied = self.color[Color::White] | self.color[Color::Black];
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
                promotion,
                ..
            } => {
                let from_bb = Bitboard::from(from);
                let to_bb = Bitboard::from(to);
                let from_to_bb = from_bb ^ to_bb;

                if let Some(promotion) = promotion {
                    self.role[role] ^= to_bb;
                    self.role[promotion] ^= to_bb;
                }

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
            Move::CastleShort => match color {
                Color::White => {
                    let king_bb = Bitboard(1 << 4) | Bitboard(1 << 6);
                    let rook_bb = Bitboard(1 << 5) | Bitboard(1 << 7);

                    self.color[color] ^= king_bb | rook_bb;
                    self.role[Role::King] ^= king_bb;
                    self.role[Role::Rook] ^= rook_bb;
                }
                Color::Black => {
                    let king_bb = Bitboard(1 << 60) | Bitboard(1 << 62);
                    let rook_bb = Bitboard(1 << 61) | Bitboard(1 << 63);

                    self.color[color] ^= king_bb | rook_bb;
                    self.role[Role::King] ^= king_bb;
                    self.role[Role::Rook] ^= rook_bb;
                }
            },
            Move::CastleLong => match color {
                Color::White => {
                    let king_bb = Bitboard(1 << 4) | Bitboard(1 << 2);
                    let rook_bb = Bitboard(1 << 0) | Bitboard(1 << 3);

                    self.color[color] ^= king_bb | rook_bb;
                    self.role[Role::King] ^= king_bb;
                    self.role[Role::Rook] ^= rook_bb;
                }
                Color::Black => {
                    let king_bb = Bitboard(1 << 60) | Bitboard(1 << 58);
                    let rook_bb = Bitboard(1 << 56) | Bitboard(1 << 59);

                    self.color[color] ^= king_bb | rook_bb;
                    self.role[Role::King] ^= king_bb;
                    self.role[Role::Rook] ^= rook_bb;
                }
            },
        }

        self.occupied = self.color[Color::White] | self.color[Color::Black];
    }

    /// Returns a `Bitboard` that indicates which sqaures are currently attacked by the pieces of
    /// the given color.
    pub fn attacked_sqaures(&self, color: Color) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;
        let blocker = self.occupied ^ (self.color[!color] & self.role[Role::King]);

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

    /// Returns a `Bitboard` indicating on which squares pieces can move to avoid check.
    /// There are 3 possible cases for this:
    /// 1. There currently is no check. A full `Bitboard` will be returned as pieces can move
    ///    anywhere.
    /// 2. The king is in check by a single piece. A `Bitboard` will be returned that contains all
    ///    sqaures a piece could move to to prevent the ceck. This includes the checker itself.
    /// 3. The king is in check by two pieces. A empty `Bitboard` will be returned as only the king
    ///    can move.
    /// Any move can now be & with the returned bitboard to prune all illegal moves in regard to
    /// checks.
    pub fn check_mask(&self, color: Color) -> Bitboard {
        let mut check_mask = Bitboard::EMPTY;
        let kings = self.role[Role::King] & self.color[color];
        let king_sqaure = Square(kings.0.trailing_zeros() as u8);

        let blocker = self.occupied;

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
        } else if (check_mask & self.color[!color]).population_count() == 1 {
            check_mask
        } else {
            Bitboard::EMPTY
        }
    }

    /// Returns a `Bitboard` representing all squares that contain pieces that are pinned on the
    /// horizontal.
    pub fn horizontal_vertical_pinmask(&self, square: Square, color: Color) -> Bitboard {
        let mut pin_mask = Bitboard::EMPTY;
        let blocker = self.occupied;
        let rook_queens = (self.role[Role::Rook] | self.role[Role::Queen]) & self.color[color];
        let pinners = rook_xray_attacks(square, blocker) & rook_queens;
        for pinner in pinners {
            pin_mask |= RAY_BETWEEN[square.to_index()][pinner];
        }

        pin_mask
    }

    /// Returns a `Bitboard` representing all squares that contain pieces that are pinned on the
    /// diagonal.
    pub fn diagonal_pinmask(&self, square: Square, color: Color) -> Bitboard {
        let mut pin_mask = Bitboard::EMPTY;
        let blocker = self.occupied;
        let bishops_queens = (self.role[Role::Bishop] | self.role[Role::Queen]) & self.color[color];
        let pinners = bishop_xray_attacks(square, blocker) & bishops_queens;
        for pinner in pinners {
            pin_mask |= RAY_BETWEEN[square.to_index()][pinner];
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
        let mut board = Board::EMPTY;
        board.put_piece_on(Piece::WHITE_PAWN, Square::D5);

        assert_eq!(board.piece_on(Square::D5).unwrap(), Piece::WHITE_PAWN);
    }

    #[test]
    fn color_on() {
        let mut board = Board::EMPTY;
        board.put_piece_on(Piece::BLACK_PAWN, Square::C4);

        assert_eq!(board.color_on(Square::C4).unwrap(), Color::Black);
    }

    #[test]
    fn role_on() {
        let mut board = Board::EMPTY;
        board.put_piece_on(Piece::BLACK_BISHOP, Square::H8);

        assert_eq!(board.role_on(Square::H8).unwrap(), Role::Bishop);
    }

    #[test]
    fn put_piece_on() {
        let mut board = Board::EMPTY;
        board.put_piece_on(Piece::WHITE_ROOK, Square::D4);
        board.put_piece_on(Piece::BLACK_QUEEN, Square::D4);

        assert_eq!(board.piece_on(Square::D4).unwrap(), Piece::BLACK_QUEEN);
    }

    #[test]
    fn make_move_no_capture() {
        let mut board = Board::EMPTY;
        board.put_piece_on(Piece::WHITE_QUEEN, Square::A1);

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
        assert_eq!(board.piece_on(Square::A8), Some(Piece::WHITE_QUEEN));
    }

    #[test]
    fn make_move_capture() {
        let mut board = Board::EMPTY;
        board.put_piece_on(Piece::WHITE_QUEEN, Square::A1);
        board.put_piece_on(Piece::BLACK_QUEEN, Square::A8);

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
        assert_eq!(board.piece_on(Square::A8), Some(Piece::WHITE_QUEEN));
    }

    #[test]
    fn make_move_promotion() {
        let mut board = Board::EMPTY;
        board.put_piece_on(Piece::WHITE_PAWN, Square::A7);

        board.make_move(
            Color::White,
            Move::Standard {
                from: Square::A7,
                to: Square::A8,
                role: Role::Pawn,
                capture: None,
                promotion: Some(Role::Queen),
                en_passant_square: None,
            },
        );

        assert_eq!(board.piece_on(Square::A7), None);
        assert_eq!(board.piece_on(Square::A8), Some(Piece::WHITE_QUEEN));
    }

    #[test]
    fn make_move_en_passant() {
        let mut board = Board::EMPTY;
        board.put_piece_on(Piece::WHITE_PAWN, Square::D5);
        board.put_piece_on(Piece::BLACK_PAWN, Square::C5);

        board.make_move(
            Color::White,
            Move::EnPassant {
                from: Square::D5,
                to: Square::C6,
                target: Square::C5,
            },
        );

        assert_eq!(board.piece_on(Square::C5), None);
        assert_eq!(board.piece_on(Square::C6), Some(Piece::WHITE_PAWN));
    }

    #[test]
    fn make_move_castle_short_white() {
        let mut board = Board::EMPTY;
        board.put_piece_on(Piece::WHITE_KING, Square::E1);
        board.put_piece_on(Piece::WHITE_ROOK, Square::H1);

        board.make_move(Color::White, Move::CastleShort);

        assert_eq!(board.piece_on(Square::E1), None);
        assert_eq!(board.piece_on(Square::G1), Some(Piece::WHITE_KING));
        assert_eq!(board.piece_on(Square::H1), None);
        assert_eq!(board.piece_on(Square::F1), Some(Piece::WHITE_ROOK));
    }

    #[test]
    fn make_move_castle_short_black() {
        let mut board = Board::EMPTY;
        board.put_piece_on(Piece::BLACK_KING, Square::E8);
        board.put_piece_on(Piece::BLACK_ROOK, Square::H8);

        board.make_move(Color::Black, Move::CastleShort);

        assert_eq!(board.piece_on(Square::E8), None);
        assert_eq!(board.piece_on(Square::G8), Some(Piece::BLACK_KING));
        assert_eq!(board.piece_on(Square::H8), None);
        assert_eq!(board.piece_on(Square::F8), Some(Piece::BLACK_ROOK));
    }

    #[test]
    fn make_move_castle_long_white() {
        let mut board = Board::EMPTY;
        board.put_piece_on(Piece::WHITE_KING, Square::E1);
        board.put_piece_on(Piece::WHITE_ROOK, Square::A1);

        board.make_move(Color::White, Move::CastleLong);

        assert_eq!(board.piece_on(Square::E1), None);
        assert_eq!(board.piece_on(Square::C1), Some(Piece::WHITE_KING));
        assert_eq!(board.piece_on(Square::A1), None);
        assert_eq!(board.piece_on(Square::D1), Some(Piece::WHITE_ROOK));
    }

    #[test]
    fn make_move_castle_long_black() {
        let mut board = Board::EMPTY;
        board.put_piece_on(Piece::BLACK_KING, Square::E8);
        board.put_piece_on(Piece::BLACK_ROOK, Square::A8);

        board.make_move(Color::Black, Move::CastleLong);

        assert_eq!(board.piece_on(Square::E8), None);
        assert_eq!(board.piece_on(Square::C8), Some(Piece::BLACK_KING));
        assert_eq!(board.piece_on(Square::A8), None);
        assert_eq!(board.piece_on(Square::D8), Some(Piece::BLACK_ROOK));
    }

    #[test]
    fn unmake_move_no_capture() {
        let mut board = Board::EMPTY;
        board.put_piece_on(Piece::WHITE_QUEEN, Square::A1);

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
        assert_eq!(board.piece_on(Square::A1), Some(Piece::WHITE_QUEEN));
    }

    #[test]
    fn unmake_move_capture() {
        let mut board = Board::EMPTY;
        board.put_piece_on(Piece::WHITE_QUEEN, Square::A1);
        board.put_piece_on(Piece::BLACK_QUEEN, Square::A8);

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

        assert_eq!(board.piece_on(Square::A8), Some(Piece::BLACK_QUEEN));
        assert_eq!(board.piece_on(Square::A1), Some(Piece::WHITE_QUEEN));
    }

    #[test]
    fn unmake_move_promotion() {
        let mut board = Board::EMPTY;
        board.put_piece_on(Piece::WHITE_QUEEN, Square::A8);

        board.unmake_move(
            Color::White,
            Move::Standard {
                from: Square::A7,
                to: Square::A8,
                role: Role::Pawn,
                capture: None,
                promotion: Some(Role::Queen),
                en_passant_square: None,
            },
        );

        assert_eq!(board.piece_on(Square::A7), Some(Piece::WHITE_PAWN));
        assert_eq!(board.piece_on(Square::A8), None);
    }

    #[test]
    fn unmake_move_en_passant() {
        let mut board = Board::EMPTY;
        board.put_piece_on(Piece::WHITE_PAWN, Square::C6);

        board.unmake_move(
            Color::White,
            Move::EnPassant {
                from: Square::D5,
                to: Square::C6,
                target: Square::C5,
            },
        );

        assert_eq!(board.piece_on(Square::D5), Some(Piece::WHITE_PAWN));
        assert_eq!(board.piece_on(Square::C5), Some(Piece::BLACK_PAWN));
    }

    #[test]
    fn unmake_move_castle_short_white() {
        let mut board = Board::EMPTY;
        board.put_piece_on(Piece::WHITE_KING, Square::G1);
        board.put_piece_on(Piece::WHITE_ROOK, Square::F1);

        board.unmake_move(Color::White, Move::CastleShort);

        assert_eq!(board.piece_on(Square::G1), None);
        assert_eq!(board.piece_on(Square::E1), Some(Piece::WHITE_KING));
        assert_eq!(board.piece_on(Square::F1), None);
        assert_eq!(board.piece_on(Square::H1), Some(Piece::WHITE_ROOK));
    }

    #[test]
    fn unmake_move_castle_short_black() {
        let mut board = Board::EMPTY;
        board.put_piece_on(Piece::BLACK_KING, Square::G8);
        board.put_piece_on(Piece::BLACK_ROOK, Square::F8);

        board.unmake_move(Color::Black, Move::CastleShort);

        assert_eq!(board.piece_on(Square::G8), None);
        assert_eq!(board.piece_on(Square::E8), Some(Piece::BLACK_KING));
        assert_eq!(board.piece_on(Square::F8), None);
        assert_eq!(board.piece_on(Square::H8), Some(Piece::BLACK_ROOK));
    }

    #[test]
    fn unmake_move_castle_long_white() {
        let mut board = Board::EMPTY;
        board.put_piece_on(Piece::WHITE_KING, Square::C1);
        board.put_piece_on(Piece::WHITE_ROOK, Square::D1);

        board.unmake_move(Color::White, Move::CastleLong);

        assert_eq!(board.piece_on(Square::C1), None);
        assert_eq!(board.piece_on(Square::E1), Some(Piece::WHITE_KING));
        assert_eq!(board.piece_on(Square::D1), None);
        assert_eq!(board.piece_on(Square::A1), Some(Piece::WHITE_ROOK));
    }

    #[test]
    fn unmake_move_castle_long_black() {
        let mut board = Board::EMPTY;
        board.put_piece_on(Piece::BLACK_KING, Square::C8);
        board.put_piece_on(Piece::BLACK_ROOK, Square::D8);

        board.unmake_move(Color::Black, Move::CastleLong);

        assert_eq!(board.piece_on(Square::C8), None);
        assert_eq!(board.piece_on(Square::E8), Some(Piece::BLACK_KING));
        assert_eq!(board.piece_on(Square::D8), None);
        assert_eq!(board.piece_on(Square::A8), Some(Piece::BLACK_ROOK));
    }
}
