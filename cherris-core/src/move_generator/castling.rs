use crate::{Bitboard, CastlingRights, Color, Move, Position, Square};
use arrayvec::ArrayVec;

pub fn generate_castling_moves(
    position: &Position,
    moves: &mut ArrayVec<Move, 256>,
    kings: Bitboard,
    blockers: Bitboard,
    attacked_squares: Bitboard,
) {
    let is_not_in_check = (kings & attacked_squares).is_empty();
    if is_not_in_check {
        let castle_path_short = match position.color_to_move {
            Color::White => Bitboard::from(Square::F1) | Bitboard::from(Square::G1),
            Color::Black => Bitboard::from(Square::F8) | Bitboard::from(Square::G8),
        };

        let castle_path_long = match position.color_to_move {
            Color::White => {
                Bitboard::from(Square::B1) | Bitboard::from(Square::C1) | Bitboard::from(Square::D1)
            }
            Color::Black => {
                Bitboard::from(Square::B8) | Bitboard::from(Square::C8) | Bitboard::from(Square::D8)
            }
        };

        let castle_path_long_attacks = match position.color_to_move {
            Color::White => Bitboard::from(Square::D1) | Bitboard::from(Square::C1),
            Color::Black => Bitboard::from(Square::D8) | Bitboard::from(Square::C8),
        };

        let path_empty = (castle_path_short & blockers).is_empty();
        let path_unattacked = (castle_path_short & attacked_squares).is_empty();
        let castling_rights = position.castling_rights[position.color_to_move];

        if path_empty
            && path_unattacked
            && (castling_rights == CastlingRights::KingSide
                || castling_rights == CastlingRights::BothSides)
        {
            let mv = Move::CastleShort;
            moves.push(mv);
        }

        let path_empty = (castle_path_long & blockers).is_empty();
        let path_unattacked = (castle_path_long_attacks & attacked_squares).is_empty();
        let castling_rights = position.castling_rights[position.color_to_move];

        if path_empty
            && path_unattacked
            && (castling_rights == CastlingRights::QueenSide
                || castling_rights == CastlingRights::BothSides)
        {
            let mv = Move::CastleLong;
            moves.push(mv);
        }
    }
}
