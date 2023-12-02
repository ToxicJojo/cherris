use cherris_core::{Color, Position, Role};

const ROLE_VALUE: [f32; Role::COUNT] = [1.0, 3.0, 3.0, 5.0, 8.0, 100.0];

pub fn eval(position: &Position) -> f32 {
    eval_material(position)
}

fn eval_material(position: &Position) -> f32 {
    let mut eval = 0.0;

    for role in Role::iter() {
        let role_count_white = position.board.count_roles(*role, Color::White) as f32;
        let role_count_black = position.board.count_roles(*role, Color::Black) as f32;

        let role_diff = role_count_white - role_count_black;
        eval += role_diff * ROLE_VALUE[role.to_index()];
    }

    eval
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn eval_material_two_pawns_up() {
        let position = Position::from_str("1k6/8/1q2n3/8/4N3/1Q2PP2/8/2K5 w - - 0 1").unwrap();

        let result = eval_material(&position);

        assert_eq!(result, ROLE_VALUE[Role::Pawn.to_index()] * 2.0);
    }

    #[test]
    fn eval_material_two_pawns_down() {
        let position =
            Position::from_str("1k1r2p1/5p2/2b1n1Q1/3p4/2p1N3/1q1BPP2/4K3/5R2 w - - 0 1").unwrap();

        let result = eval_material(&position);

        assert_eq!(result, ROLE_VALUE[Role::Pawn.to_index()] * -2.0);
    }
}
