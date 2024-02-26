pub mod evaluation;
mod material;
mod psqt;

use self::{evaluation::Evaluation, material::eval_material, psqt::eval_psqt};
use cherris_core::Position;

pub fn eval(position: &Position) -> Evaluation {
    let mut eval = Evaluation::DRAW;

    eval += eval_material(position);
    eval += eval_psqt(position);

    eval
}

pub use material::ROLE_VALUE;
