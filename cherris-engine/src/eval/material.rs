use cherris_core::{Color, Position, Role};

use super::evaluation::Evaluation;

pub const ROLE_VALUE: [i16; Role::COUNT] = [100, 300, 300, 500, 800, 10000];

pub fn eval_material(position: &Position) -> Evaluation {
    let mut eval = 0;

    for role in Role::iter() {
        let role_count_white = position.board.count_roles(*role, Color::White) as i16;
        let role_count_black = position.board.count_roles(*role, Color::Black) as i16;

        let role_diff = role_count_white - role_count_black;
        eval += role_diff * ROLE_VALUE[role];
    }

    Evaluation::new(eval)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn eval_material_two_pawns_up() {
        let position = Position::from_str("1k6/8/1q2n3/8/4N3/1Q2PP2/8/2K5 w - - 0 1").unwrap();

        let result = eval_material(&position);

        assert_eq!(result, Evaluation::new(ROLE_VALUE[Role::Pawn] * 2));
    }

    #[test]
    fn eval_material_two_pawns_down() {
        let position =
            Position::from_str("1k1r2p1/5p2/2b1n1Q1/3p4/2p1N3/1q1BPP2/4K3/5R2 w - - 0 1").unwrap();

        let result = eval_material(&position);

        assert_eq!(result, Evaluation::new(ROLE_VALUE[Role::Pawn] * -2));
    }
}
