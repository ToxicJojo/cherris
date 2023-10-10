use crate::{Move, Position, Role, KNIGHT_MOVES};

pub fn generate_moves(position: Position) -> Vec<Move> {
    let mut moves = Vec::new();

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

    moves
}
