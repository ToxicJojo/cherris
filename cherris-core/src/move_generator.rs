use arrayvec::ArrayVec;

use crate::{
    bishop_attacks, king_attacks, knight_attacks, pawn_attacks, queen_attacks, rook_attacks,
    Bitboard, CastlingRights, Color, Move, Position, Role, Square,
};

pub fn generate_moves(position: &Position, moves: &mut ArrayVec<Move, 256>) {
    let blockers = position.board.occupied;
    let empty = !blockers;

    let attacked_squares = position.board.attacked_sqaures(!position.color_to_move);
    let check_mask = position.board.check_mask(position.color_to_move);

    let kings = position.board.role[Role::King] & position.board.color[position.color_to_move];
    let king = Square(kings.0.trailing_zeros() as u8);
    let hv_pins = position
        .board
        .horizontal_vertical_pinmask(king, !position.color_to_move);
    let diag_ping = position
        .board
        .diagonal_pinmask(king, !position.color_to_move);

    let pawns = position.board.role[Role::Pawn] & position.board.color[position.color_to_move];
    let pawns_attack = pawns & !hv_pins;
    let pawns_pinned_diag = pawns_attack & diag_ping;

    let pawns_pinned_promotion =
        pawns_pinned_diag & Bitboard::PRE_PROMOTION_RANK[position.color_to_move];
    let pawns_pinned_no_promotion = pawns_pinned_diag ^ pawns_pinned_promotion;

    let pawns_attack_unpinned = pawns_attack ^ pawns_pinned_diag;
    let pawns_attack_unpinned_promotion =
        pawns_attack_unpinned & Bitboard::PRE_PROMOTION_RANK[position.color_to_move];
    let pawns_attack_unpinned_no_promotion =
        pawns_attack_unpinned ^ pawns_attack_unpinned_promotion;

    let pawns_walk = pawns & !diag_ping;
    let pawns_forward = match position.color_to_move {
        Color::White => pawns_walk & (empty >> 8),
        Color::Black => pawns_walk & (empty << 8),
    };
    let pawns_pinned_hv = pawns_forward & hv_pins;
    let pawns_forward_pinned = pawns_forward & pawns_pinned_hv;
    let pawns_forward_unpinned = pawns_forward ^ pawns_forward_pinned;

    let pawns_fw = match position.color_to_move {
        Color::White => (pawns_forward_unpinned << 8) | (pawns_forward_pinned << 8 & hv_pins),
        Color::Black => (pawns_forward_unpinned >> 8) | (pawns_forward_pinned >> 8 & hv_pins),
    } & check_mask;

    let pawns_fw_promotion = pawns_fw & Bitboard::PROMOTION_RANK[position.color_to_move];
    let pawns_fw_no_promotion = pawns_fw ^ pawns_fw_promotion;

    let pawns_push = match position.color_to_move {
        Color::White => pawns_forward & Bitboard::SECOND_RANK & (empty >> 16),
        Color::Black => pawns_forward & Bitboard::SEVENTH_RANK & (empty << 16),
    };

    let pawns_push_pinned = pawns_push & pawns_pinned_hv;
    let pawns_push_unpinned = pawns_push ^ pawns_push_pinned;
    let pawns_push = match position.color_to_move {
        Color::White => (pawns_push_unpinned << 16) | (pawns_push_pinned << 16 & hv_pins),
        Color::Black => (pawns_push_unpinned >> 16) | (pawns_push_pinned >> 16 & hv_pins),
    } & check_mask;

    let en_passant_bb = position
        .en_passant_square
        .map_or(Bitboard::EMPTY, Bitboard::from);

    for from in pawns_attack_unpinned_no_promotion {
        let attacks = pawn_attacks(from, position.color_to_move) & check_mask;

        let attacks_en_passant = attacks & en_passant_bb & !diag_ping;
        let attacks = attacks & position.board.color[!position.color_to_move];

        add_attacks(attacks, from, Role::Pawn, position, moves);

        for to in attacks_en_passant {
            let target = match position.color_to_move {
                Color::White => Square(to.0 - 8),
                Color::Black => Square(to.0 + 8),
            };

            let from_bb = Bitboard::from(from);
            let target_bb = Bitboard::from(target);

            let occ = blockers & !from_bb & !target_bb;
            let king_sees = rook_attacks(king, occ);
            if !(king_sees
                & position.board.color[!position.color_to_move]
                & (position.board.role[Role::Rook] | position.board.role[Role::Queen]))
                .is_empty()
            {
                continue;
            }

            let mv = Move::EnPassant { from, to, target };

            moves.push(mv);
        }
    }

    for from in pawns_attack_unpinned_promotion {
        let attacks = pawn_attacks(from, position.color_to_move) & check_mask;

        let attacks = attacks & position.board.color[!position.color_to_move];

        for to in attacks {
            generate_promotion_move(from, to, position, moves);
        }
    }

    for from in pawns_pinned_no_promotion {
        let attacks = pawn_attacks(from, position.color_to_move) & diag_ping & check_mask;

        let attacks_en_passant = attacks & en_passant_bb & !diag_ping;
        let attacks = attacks & position.board.color[!position.color_to_move];

        add_attacks(attacks, from, Role::Pawn, position, moves);

        for to in attacks_en_passant {
            let target = match position.color_to_move {
                Color::White => Square(to.0 - 8),
                Color::Black => Square(to.0 + 8),
            };

            let from_bb = Bitboard::from(from);
            let target_bb = Bitboard::from(target);

            let occ = blockers & !from_bb & !target_bb;
            let king_sees = rook_attacks(king, occ);
            if !(king_sees
                & position.board.color[!position.color_to_move]
                & (position.board.role[Role::Rook] | position.board.role[Role::Queen]))
                .is_empty()
            {
                continue;
            }

            let mv = Move::EnPassant { from, to, target };

            moves.push(mv);
        }
    }

    for from in pawns_pinned_promotion {
        let attacks = pawn_attacks(from, position.color_to_move) & diag_ping & check_mask;

        let attacks = attacks & position.board.color[!position.color_to_move];
        for to in attacks {
            generate_promotion_move(from, to, position, moves);
        }
    }

    for to in pawns_fw_no_promotion {
        let from = match position.color_to_move {
            Color::White => Square(to.0 - 8),
            Color::Black => Square(to.0 + 8),
        };

        let mv = Move::Standard {
            role: Role::Pawn,
            from,
            to,
            capture: None,
            promotion: None,
            en_passant_square: None,
        };

        unsafe {
            moves.push_unchecked(mv);
        }
    }

    for to in pawns_fw_promotion {
        let from = match position.color_to_move {
            Color::White => Square(to.0 - 8),
            Color::Black => Square(to.0 + 8),
        };

        generate_promotion_move(from, to, position, moves);
    }

    for to in pawns_push {
        let from = match position.color_to_move {
            Color::White => Square(to.0 - 16),
            Color::Black => Square(to.0 + 16),
        };

        let en_passant = match position.color_to_move {
            Color::White => Square(from.0 + 8),
            Color::Black => Square(from.0 - 8),
        };

        let mv = Move::Standard {
            role: Role::Pawn,
            from,
            to,
            capture: None,
            promotion: None,
            en_passant_square: Some(en_passant),
        };

        unsafe {
            moves.push_unchecked(mv);
        }
    }

    let kings = position.board.role[Role::King] & position.board.color[position.color_to_move];

    for from in kings {
        let mut attacks = king_attacks(from);
        attacks &= !position.board.color[position.color_to_move];
        attacks &= !attacked_squares;

        add_attacks(attacks, from, Role::King, position, moves);
    }

    let knights = position.board.role[Role::Knight] & position.board.color[position.color_to_move];
    let knights = knights & !(hv_pins | diag_ping);

    for from in knights {
        let mut attacks = knight_attacks(from);
        attacks &= !position.board.color[position.color_to_move] & check_mask;

        add_attacks(attacks, from, Role::Knight, position, moves);
    }

    let rooks = position.board.role[Role::Rook] & position.board.color[position.color_to_move];
    let rooks = rooks & !diag_ping;
    let rooks_pinned = rooks & hv_pins;
    let rooks_unpinned = rooks ^ rooks_pinned;

    for from in rooks_unpinned {
        let mut attacks = rook_attacks(from, blockers);
        attacks &= !position.board.color[position.color_to_move] & check_mask;

        add_attacks(attacks, from, Role::Rook, position, moves);
    }

    for from in rooks_pinned {
        let mut attacks = rook_attacks(from, blockers);
        attacks &= !position.board.color[position.color_to_move] & check_mask;
        attacks &= hv_pins;

        add_attacks(attacks, from, Role::Rook, position, moves);
    }

    let bishops = position.board.role[Role::Bishop] & position.board.color[position.color_to_move];
    let bishops = bishops & !hv_pins;
    let bishops_pinned = bishops & diag_ping;
    let bishops_unpinned = bishops ^ bishops_pinned;

    for from in bishops_unpinned {
        let mut attacks = bishop_attacks(from, blockers);
        attacks &= !position.board.color[position.color_to_move] & check_mask;

        add_attacks(attacks, from, Role::Bishop, position, moves);
    }

    for from in bishops_pinned {
        let mut attacks = bishop_attacks(from, blockers);
        attacks &= !position.board.color[position.color_to_move] & check_mask;
        attacks &= diag_ping;

        add_attacks(attacks, from, Role::Bishop, position, moves);
    }

    let queens = position.board.role[Role::Queen] & position.board.color[position.color_to_move];
    let queens_pinned_hv = queens & hv_pins;
    let queens_pinned_diag = queens & diag_ping;
    let queens_unpinned = queens ^ (queens_pinned_hv | queens_pinned_diag);

    for from in queens_unpinned {
        let mut attacks = queen_attacks(from, blockers);
        attacks &= !position.board.color[position.color_to_move] & check_mask;

        add_attacks(attacks, from, Role::Queen, position, moves);
    }

    for from in queens_pinned_diag {
        let mut attacks = queen_attacks(from, blockers);
        attacks &= !position.board.color[position.color_to_move] & check_mask & diag_ping;

        add_attacks(attacks, from, Role::Queen, position, moves);
    }

    for from in queens_pinned_hv {
        let mut attacks = queen_attacks(from, blockers);
        attacks &= !position.board.color[position.color_to_move] & check_mask & hv_pins;

        add_attacks(attacks, from, Role::Queen, position, moves);
    }

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
        let castling_rights = position.castling_rights[position.color_to_move.to_index()];

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
        let castling_rights = position.castling_rights[position.color_to_move.to_index()];

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

#[inline]
fn add_attacks(
    attacks: Bitboard,
    from: Square,
    role: Role,
    position: &Position,
    moves: &mut ArrayVec<Move, 256>,
) {
    for to in attacks {
        let mv = Move::Standard {
            role,
            from,
            to,
            capture: position.board.role_on(to),
            promotion: None,
            en_passant_square: None,
        };

        unsafe {
            moves.push_unchecked(mv);
        }
    }
}

#[inline]
fn generate_promotion_move(
    from: Square,
    to: Square,
    position: &Position,
    moves: &mut ArrayVec<Move, 256>,
) {
    for role in Role::iter().skip(1).take(4) {
        let mv = Move::Standard {
            role: Role::Pawn,
            from,
            to,
            capture: position.board.role_on(to),
            promotion: Some(*role),
            en_passant_square: None,
        };

        unsafe {
            moves.push_unchecked(mv);
        }
    }
}
