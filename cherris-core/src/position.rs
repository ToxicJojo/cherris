use std::str::FromStr;

use crate::{Bitboard, Board, Color, File, Rank, Role, Square};

/// Represents a chess position.
pub struct Position {
    pub board: Board,
    pub color_to_move: Color,
    pub en_passant_square: Option<Square>,
    pub halfmove_clock: u8,
    pub fullmove_clock: usize,
}

#[derive(Debug)]
pub enum ParseFenError {
    InvalidFen,
}

impl FromStr for Position {
    type Err = ParseFenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        let pieces_str = parts[0];
        let color_to_move = parts[1];
        let en_passant = parts[3];
        let halfmove_clock = parts[4];
        let fullmove_clock = parts[5];

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
                _ => return Err(ParseFenError::InvalidFen),
            }

            file = file.right();
        }

        let board = Board { role, color };

        Ok(Position {
            board,
            color_to_move: Color::White,
            en_passant_square: None,
            halfmove_clock: 0,
            fullmove_clock: 0,
        })
    }
}
