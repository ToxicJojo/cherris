use super::{add_attacks, generate_promotion_move};
use crate::{pawn_attacks, rook_attacks, Bitboard, Color, Move, MoveList, Position, Role, Square};

pub fn generate_pawn_moves(
    position: &Position,
    moves: &mut MoveList,
    empty: Bitboard,
    hv_pins: Bitboard,
    diag_pins: Bitboard,
    check_mask: Bitboard,
    king: Square,
) {
    let blockers = position.board.occupied;
    let pawns = position.board.role[Role::Pawn] & position.board.color[position.color_to_move];
    let pawns_attack = pawns & !hv_pins;
    let pawns_pinned_diag = pawns_attack & diag_pins;

    let pawns_pinned_promotion =
        pawns_pinned_diag & Bitboard::PRE_PROMOTION_RANK[position.color_to_move];
    let pawns_pinned_no_promotion = pawns_pinned_diag ^ pawns_pinned_promotion;

    let pawns_attack_unpinned = pawns_attack ^ pawns_pinned_diag;
    let pawns_attack_unpinned_promotion =
        pawns_attack_unpinned & Bitboard::PRE_PROMOTION_RANK[position.color_to_move];
    let pawns_attack_unpinned_no_promotion =
        pawns_attack_unpinned ^ pawns_attack_unpinned_promotion;

    let pawns_walk = pawns & !diag_pins;
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

    let en_passant_target = match position.color_to_move {
        Color::White => en_passant_bb >> 8,
        Color::Black => en_passant_bb << 8,
    };

    let ep_check_mask = match position.color_to_move {
        Color::White => (check_mask & en_passant_target) << 8,
        Color::Black => (check_mask & en_passant_target) >> 8,
    };

    for from in pawns_attack_unpinned_no_promotion {
        let attacks = pawn_attacks(from, position.color_to_move) & check_mask;
        let attacks = attacks & position.board.color[!position.color_to_move];
        add_attacks(attacks, from, Role::Pawn, position, moves);

        let attacks_en_passant = pawn_attacks(from, position.color_to_move) & ep_check_mask;
        let attacks_en_passant = attacks_en_passant & en_passant_bb & !diag_pins;

        for to in attacks_en_passant {
            let target = en_passant_target.to_square();
            let from_bb = Bitboard::from(from);

            let occ = blockers & !from_bb & !en_passant_target;
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
        let attacks = pawn_attacks(from, position.color_to_move) & diag_pins & check_mask;
        let attacks = attacks & position.board.color[!position.color_to_move];
        add_attacks(attacks, from, Role::Pawn, position, moves);

        let attacks_en_passant =
            pawn_attacks(from, position.color_to_move) & diag_pins & ep_check_mask;
        let attacks_en_passant = attacks_en_passant & en_passant_bb & !diag_pins;

        for to in attacks_en_passant {
            let target = en_passant_target.to_square();
            let from_bb = Bitboard::from(from);

            let occ = blockers & !from_bb & !en_passant_target;
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
        let attacks = pawn_attacks(from, position.color_to_move) & diag_pins & check_mask;

        let attacks = attacks & position.board.color[!position.color_to_move];
        for to in attacks {
            generate_promotion_move(from, to, position, moves);
        }
    }

    for to in pawns_fw_no_promotion {
        let to_index = to.to_index() as u8;
        let from = match position.color_to_move {
            Color::White => Square::from_index(to_index - 8),
            Color::Black => Square::from_index(to_index + 8),
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
        let to_index = to.to_index() as u8;
        let from = match position.color_to_move {
            Color::White => Square::from_index(to_index - 8),
            Color::Black => Square::from_index(to_index + 8),
        };

        generate_promotion_move(from, to, position, moves);
    }

    for to in pawns_push {
        let to_index = to.to_index() as u8;
        let from = match position.color_to_move {
            Color::White => Square::from_index(to_index - 16),
            Color::Black => Square::from_index(to_index + 16),
        };

        let from_index = from.to_index() as u8;
        let en_passant = match position.color_to_move {
            Color::White => Square::from_index(from_index + 8),
            Color::Black => Square::from_index(from_index - 8),
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
}

pub fn generate_loud_pawn_moves(
    position: &Position,
    moves: &mut MoveList,
    hv_pins: Bitboard,
    diag_pins: Bitboard,
    check_mask: Bitboard,
    king: Square,
) {
    let blockers = position.board.occupied;
    let pawns = position.board.role[Role::Pawn] & position.board.color[position.color_to_move];
    let pawns_attack = pawns & !hv_pins;
    let pawns_pinned_diag = pawns_attack & diag_pins;

    let pawns_pinned_promotion =
        pawns_pinned_diag & Bitboard::PRE_PROMOTION_RANK[position.color_to_move];
    let pawns_pinned_no_promotion = pawns_pinned_diag ^ pawns_pinned_promotion;

    let pawns_attack_unpinned = pawns_attack ^ pawns_pinned_diag;
    let pawns_attack_unpinned_promotion =
        pawns_attack_unpinned & Bitboard::PRE_PROMOTION_RANK[position.color_to_move];
    let pawns_attack_unpinned_no_promotion =
        pawns_attack_unpinned ^ pawns_attack_unpinned_promotion;

    let en_passant_bb = position
        .en_passant_square
        .map_or(Bitboard::EMPTY, Bitboard::from);

    let en_passant_target = match position.color_to_move {
        Color::White => en_passant_bb >> 8,
        Color::Black => en_passant_bb << 8,
    };

    let ep_check_mask = match position.color_to_move {
        Color::White => (check_mask & en_passant_target) << 8,
        Color::Black => (check_mask & en_passant_target) >> 8,
    };

    for from in pawns_attack_unpinned_no_promotion {
        let attacks = pawn_attacks(from, position.color_to_move) & check_mask;
        let attacks = attacks & position.board.color[!position.color_to_move];
        add_attacks(attacks, from, Role::Pawn, position, moves);

        let attacks_en_passant = pawn_attacks(from, position.color_to_move) & ep_check_mask;
        let attacks_en_passant = attacks_en_passant & en_passant_bb & !diag_pins;

        for to in attacks_en_passant {
            let target = en_passant_target.to_square();
            let from_bb = Bitboard::from(from);

            let occ = blockers & !from_bb & !en_passant_target;
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
        let attacks = pawn_attacks(from, position.color_to_move) & diag_pins & check_mask;
        let attacks = attacks & position.board.color[!position.color_to_move];
        add_attacks(attacks, from, Role::Pawn, position, moves);

        let attacks_en_passant =
            pawn_attacks(from, position.color_to_move) & diag_pins & ep_check_mask;
        let attacks_en_passant = attacks_en_passant & en_passant_bb & !diag_pins;

        for to in attacks_en_passant {
            let target = en_passant_target.to_square();
            let from_bb = Bitboard::from(from);

            let occ = blockers & !from_bb & !en_passant_target;
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
}
