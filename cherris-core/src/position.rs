use std::{fmt::Display, str::FromStr};

use crate::{
    generate_moves, Bitboard, Board, CastlingRights, Color, Error, File, Move, MoveList, Rank,
    Role, Square, Zobrist,
};

/// Represents a chess position.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub board: Board,
    pub color_to_move: Color,
    pub en_passant_square: Option<Square>,
    pub halfmove_clock: u8,
    pub fullmove_number: usize,
    pub castling_rights: [CastlingRights; Color::COUNT],
    pub zobrist: Zobrist,
}

impl Position {
    pub const STARTING_FEN: &'static str =
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    pub fn make_move(&mut self, chess_move: Move) {
        self.board.make_move(self.color_to_move, chess_move);
        self.zobrist.update_castling_right(self.castling_rights);

        self.en_passant_square = match chess_move {
            Move::Standard {
                en_passant_square, ..
            } => en_passant_square,
            _ => None,
        };

        if self.color_to_move == Color::Black {
            self.fullmove_number += 1;
        }

        if let Move::Standard { role, capture, .. } = chess_move {
            if role != Role::Pawn && capture.is_some() {
                self.halfmove_clock += 1;
            }
        }

        self.castling_rights[self.color_to_move] = match chess_move {
            Move::Standard { from, role, .. } => {
                if role == Role::King {
                    self.castling_rights[self.color_to_move] = CastlingRights::NoSide;
                }
                if role == Role::Rook {
                    match self.color_to_move {
                        Color::White => {
                            if from == Square::A1 {
                                self.castling_rights[self.color_to_move].remove_queen_side();
                            }

                            if from == Square::H1 {
                                self.castling_rights[self.color_to_move].remove_king_side();
                            }
                        }
                        Color::Black => {
                            if from == Square::A8 {
                                self.castling_rights[self.color_to_move].remove_queen_side();
                            }

                            if from == Square::H8 {
                                self.castling_rights[self.color_to_move].remove_king_side();
                            }
                        }
                    }
                };

                self.castling_rights[self.color_to_move]
            }
            Move::EnPassant { .. } => self.castling_rights[self.color_to_move],
            Move::CastleLong => CastlingRights::NoSide,
            Move::CastleShort => CastlingRights::NoSide,
        };

        if let Move::Standard {
            to,
            capture: Some(role),
            ..
        } = chess_move
        {
            if role == Role::Rook {
                match !self.color_to_move {
                    Color::White => {
                        if to == Square::A1 {
                            self.castling_rights[!self.color_to_move].remove_queen_side();
                        }

                        if to == Square::H1 {
                            self.castling_rights[!self.color_to_move].remove_king_side();
                        }
                    }
                    Color::Black => {
                        if to == Square::A8 {
                            self.castling_rights[!self.color_to_move].remove_queen_side();
                        }

                        if to == Square::H8 {
                            self.castling_rights[!self.color_to_move].remove_king_side();
                        }
                    }
                }
            }
        }

        self.zobrist.update_castling_right(self.castling_rights);
        self.zobrist.update(chess_move, self.color_to_move);
        self.color_to_move = !self.color_to_move;
    }

    pub fn legal_moves(&self) -> MoveList {
        let mut moves = MoveList::new();
        generate_moves(self, &mut moves);
        moves
    }

    /// Checks if the color that is moving is in check.
    pub fn is_in_check(&self) -> bool {
        let king_sqaure = self.board.role[Role::King] & self.board.color[self.color_to_move];
        let attacked_squares = self.board.attacked_sqaures(!self.color_to_move);

        !(king_sqaure & attacked_squares).is_empty()
    }

    /// Checks if the color that is moving is in checkmake.
    pub fn is_checkmate(&self) -> bool {
        let moves = self.legal_moves();
        let check = self.is_in_check();

        return moves.is_empty() && check;
    }

    /// Checks if the color that is moving is in stalemate.
    pub fn is_stalemate(&self) -> bool {
        let moves = self.legal_moves();
        let check = self.is_in_check();

        return moves.is_empty() && !check;
    }
}

impl FromStr for Position {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        let pieces_str = parts[0];
        let color_to_move = parts[1];
        let castling_rights = parts[2];
        let en_passant = parts[3];
        let halfmove_clock = parts[4];
        let fullmove_number = parts[5];

        let mut file = File::A;
        let mut rank = Rank::Eigth;
        let mut role = [Bitboard::EMPTY; Role::COUNT];
        let mut color = [Bitboard::EMPTY; Color::COUNT];

        for char in pieces_str.chars() {
            let sqaure = Square::from((file, rank));

            match char {
                '/' => {
                    rank = rank.down();
                    file = File::A;
                    continue;
                }
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                    for _ in 0..char.to_digit(10).unwrap() {
                        file = file.right();
                    }
                    continue;
                }
                'p' => {
                    role[Role::Pawn] |= Bitboard::from(sqaure);
                    color[Color::Black] |= Bitboard::from(sqaure);
                }
                'P' => {
                    role[Role::Pawn] |= Bitboard::from(sqaure);
                    color[Color::White] |= Bitboard::from(sqaure);
                }

                'n' => {
                    role[Role::Knight] |= Bitboard::from(sqaure);
                    color[Color::Black] |= Bitboard::from(sqaure);
                }
                'N' => {
                    role[Role::Knight] |= Bitboard::from(sqaure);
                    color[Color::White] |= Bitboard::from(sqaure);
                }

                'b' => {
                    role[Role::Bishop] |= Bitboard::from(sqaure);
                    color[Color::Black] |= Bitboard::from(sqaure);
                }
                'B' => {
                    role[Role::Bishop] |= Bitboard::from(sqaure);
                    color[Color::White] |= Bitboard::from(sqaure);
                }

                'r' => {
                    role[Role::Rook] |= Bitboard::from(sqaure);
                    color[Color::Black] |= Bitboard::from(sqaure);
                }
                'R' => {
                    role[Role::Rook] |= Bitboard::from(sqaure);
                    color[Color::White] |= Bitboard::from(sqaure);
                }

                'q' => {
                    role[Role::Queen] |= Bitboard::from(sqaure);
                    color[Color::Black] |= Bitboard::from(sqaure);
                }
                'Q' => {
                    role[Role::Queen] |= Bitboard::from(sqaure);
                    color[Color::White] |= Bitboard::from(sqaure);
                }

                'k' => {
                    role[Role::King] |= Bitboard::from(sqaure);
                    color[Color::Black] |= Bitboard::from(sqaure);
                }
                'K' => {
                    role[Role::King] |= Bitboard::from(sqaure);
                    color[Color::White] |= Bitboard::from(sqaure);
                }
                _ => return Err(Error::InvalidFen),
            }

            file = file.right();
        }

        let board = Board {
            role,
            color,
            occupied: color[Color::White] | color[Color::Black],
        };

        let color_to_move = Color::from_str(color_to_move)?;
        let castling_rights = CastlingRights::from_fen_str(castling_rights);

        let mut en_passant_square = None;
        if en_passant != "-" {
            let sqaure = Square::from_str(en_passant)?;
            en_passant_square = Some(sqaure);
        }

        let halfmove_clock = halfmove_clock
            .parse::<u8>()
            .map_err(|_| Error::InvalidFen)?;

        let fullmove_number = fullmove_number
            .parse::<usize>()
            .map_err(|_| Error::InvalidFen)?;

        Ok(Position {
            board,
            color_to_move,
            en_passant_square,
            halfmove_clock,
            fullmove_number,
            castling_rights,
            zobrist: Zobrist::DEFAULT,
        })
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in Rank::iter().rev() {
            let mut empty_count = 0;
            for file in File::iter() {
                let square = Square::from((*file, *rank));
                let piece = self.board.piece_on(square);
                match piece {
                    Some(piece) => {
                        if empty_count > 0 {
                            write!(f, "{}", empty_count)?;
                            empty_count = 0;
                        }
                        write!(f, "{}", piece)?
                    }
                    None => empty_count += 1,
                }
            }
            if empty_count > 0 {
                write!(f, "{}", empty_count)?;
            }
            if *rank != Rank::First {
                write!(f, "/")?;
            }
        }

        write!(f, " {}", self.color_to_move)?;

        let white_castling = self.castling_rights[Color::White];
        let black_castling = self.castling_rights[Color::Black];
        if white_castling == CastlingRights::NoSide && black_castling == CastlingRights::NoSide {
            write!(f, " -")?;
        } else {
            write!(f, " {}", white_castling.to_string().to_uppercase())?;
            write!(f, "{}", black_castling)?;
        }

        match self.en_passant_square {
            Some(ep_square) => write!(f, " {}", ep_square)?,
            None => write!(f, " -")?,
        }

        write!(f, " {}", self.halfmove_clock)?;
        write!(f, " {}", self.fullmove_number)?;

        Ok(())
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::from_str(Position::STARTING_FEN).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_in_check_true() {
        let pos =
            Position::from_str("rnbqkbnr/ppppp1pp/8/5p1Q/4P3/8/PPPP1PPP/RNB1KBNR b KQkq - 1 2")
                .unwrap();
        assert_eq!(pos.is_in_check(), true);
    }

    #[test]
    fn is_in_check_false() {
        let pos =
            Position::from_str("rnbqkbnr/ppppp1pp/8/5p1Q/4P3/8/PPPP1PPP/RNB1KBNR w KQkq - 1 2")
                .unwrap();
        assert_eq!(pos.is_in_check(), false);
    }

    #[test]
    fn is_checkmate_true() {
        let pos =
            Position::from_str("rnbqkbnr/ppppp2p/8/5ppQ/4P3/3P4/PPP2PPP/RNB1KBNR b KQkq - 1 3")
                .unwrap();
        assert_eq!(pos.is_checkmate(), true);
    }

    #[test]
    fn is_checkmate_false() {
        let pos =
            Position::from_str("2k5/pp1rn2p/4Q3/1Nqp1p2/2P5/8/PP3PPP/3R2K1 b - - 0 22").unwrap();
        assert_eq!(pos.is_checkmate(), false);
    }

    #[test]
    fn is_stalemate_true() {
        let pos = Position::from_str("7k/5K2/6Q1/8/8/8/8/8 b - - 0 1").unwrap();
        assert_eq!(pos.is_stalemate(), true);
    }

    #[test]
    fn is_stalemate_false() {
        let pos = Position::from_str("7k/5K2/6Q1/8/8/8/8/8 w - - 0 1").unwrap();
        assert_eq!(pos.is_stalemate(), false);
    }

    #[test]
    fn display_starting_pos() {
        assert_eq!(Position::default().to_string(), Position::STARTING_FEN);
    }

    #[test]
    fn display_1() {
        let fen = "3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1";
        let pos = Position::from_str(fen).unwrap();
        assert_eq!(pos.to_string(), fen);
    }

    #[test]
    fn display_2() {
        let fen = "7k/3p4/8/8/3P4/8/8/K7 w - - 0 1";
        let pos = Position::from_str(fen).unwrap();
        assert_eq!(pos.to_string(), fen);
    }

    #[test]
    fn display_3() {
        let fen = "r3k2r/8/8/8/8/8/8/1R2K2R b Kkq - 0 1";
        let pos = Position::from_str(fen).unwrap();
        assert_eq!(pos.to_string(), fen);
    }
}
