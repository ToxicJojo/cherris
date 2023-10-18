use crate::{bishop_attacks, rook_attacks, Color, Move, Position, Role, KING_MOVES, KNIGHT_MOVES};

pub fn generate_moves(position: &Position) -> Vec<Move> {
    let mut moves = Vec::with_capacity(256);

    let kings = position.board.role[Role::King] & position.board.color[position.color_to_move];

    for from in kings {
        let mut attacks = KING_MOVES[from.to_index()];
        attacks &= !position.board.color[position.color_to_move];

        for to in attacks {
            let mv = Move {
                role: Role::King,
                from,
                to,
                capture: position.board.piece_on(to).map(|piece| piece.role),
                promotion: None,
            };

            moves.push(mv);
        }
    }

    let knights = position.board.role[Role::Knight] & position.board.color[position.color_to_move];

    for from in knights {
        let mut attacks = KNIGHT_MOVES[from.to_index()];
        attacks &= !position.board.color[position.color_to_move];

        for to in attacks {
            let mv = Move {
                role: Role::Knight,
                from,
                to,
                capture: position.board.piece_on(to).map(|piece| piece.role),
                promotion: None,
            };

            moves.push(mv);
        }
    }

    let rooks = position.board.role[Role::Rook] & position.board.color[position.color_to_move];
    let blockers = position.board.color[Color::White] | position.board.color[Color::Black];

    for from in rooks {
        let mut attacks = rook_attacks(from, blockers);
        attacks &= !position.board.color[position.color_to_move];

        for to in attacks {
            let mv = Move {
                role: Role::Rook,
                from,
                to,
                capture: position.board.piece_on(to).map(|piece| piece.role),
                promotion: None,
            };

            moves.push(mv);
        }
    }

    let bishops = position.board.role[Role::Bishop] & position.board.color[position.color_to_move];

    for from in bishops {
        let mut attacks = bishop_attacks(from, blockers);
        attacks &= !position.board.color[position.color_to_move];

        for to in attacks {
            let mv = Move {
                role: Role::Bishop,
                from,
                to,
                capture: position.board.piece_on(to).map(|piece| piece.role),
                promotion: None,
            };

            moves.push(mv);
        }
    }

    let queens = position.board.role[Role::Queen] & position.board.color[position.color_to_move];

    for from in queens {
        let mut attacks = bishop_attacks(from, blockers) | rook_attacks(from, blockers);
        attacks &= !position.board.color[position.color_to_move];

        for to in attacks {
            let mv = Move {
                role: Role::Queen,
                from,
                to,
                capture: position.board.piece_on(to).map(|piece| piece.role),
                promotion: None,
            };

            moves.push(mv);
        }
    }

    moves
}
