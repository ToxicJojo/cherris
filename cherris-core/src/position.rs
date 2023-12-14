use std::str::FromStr;

use arrayvec::ArrayVec;

use crate::{
    generate_moves, Bitboard, Board, CastlingRights, Color, Error, File, Move, Rank, Role, Square,
    Zobrist,
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

        self.castling_rights[self.color_to_move.to_index()] = match chess_move {
            Move::Standard { from, role, .. } => {
                if role == Role::King {
                    self.castling_rights[self.color_to_move.to_index()] = CastlingRights::NoSide;
                }
                if role == Role::Rook {
                    match self.color_to_move {
                        Color::White => {
                            if from == Square::A1 {
                                self.castling_rights[self.color_to_move.to_index()]
                                    .remove_queen_side();
                            }

                            if from == Square::H1 {
                                self.castling_rights[self.color_to_move.to_index()]
                                    .remove_king_side();
                            }
                        }
                        Color::Black => {
                            if from == Square::A8 {
                                self.castling_rights[self.color_to_move.to_index()]
                                    .remove_queen_side();
                            }

                            if from == Square::H8 {
                                self.castling_rights[self.color_to_move.to_index()]
                                    .remove_king_side();
                            }
                        }
                    }
                };

                self.castling_rights[self.color_to_move.to_index()]
            }
            Move::EnPassant { .. } => self.castling_rights[self.color_to_move.to_index()],
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
                            self.castling_rights[(!self.color_to_move).to_index()]
                                .remove_queen_side();
                        }

                        if to == Square::H1 {
                            self.castling_rights[(!self.color_to_move).to_index()]
                                .remove_king_side();
                        }
                    }
                    Color::Black => {
                        if to == Square::A8 {
                            self.castling_rights[(!self.color_to_move).to_index()]
                                .remove_queen_side();
                        }

                        if to == Square::H8 {
                            self.castling_rights[(!self.color_to_move).to_index()]
                                .remove_king_side();
                        }
                    }
                }
            }
        }

        self.color_to_move = !self.color_to_move;
        self.zobrist = Zobrist::from(&*self);
    }

    pub fn legal_moves(&self) -> ArrayVec<Move, 256> {
        let mut moves = ArrayVec::<Move, 256>::new();
        generate_moves(self, &mut moves);
        moves
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

        let white_castling = CastlingRights::from_str(castling_rights, Color::White);
        let black_castling = CastlingRights::from_str(castling_rights, Color::Black);

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
            castling_rights: [white_castling, black_castling],
            zobrist: Zobrist::DEFAULT,
        })
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::from_str(Position::STARTING_FEN).unwrap()
    }
}
