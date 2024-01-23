mod material;
mod psqt;

use self::{material::eval_material, psqt::eval_psqt};
use cherris_core::Position;

pub fn eval(position: &Position) -> i16 {
    let mut eval = 0;

    eval += eval_material(position);
    eval += eval_psqt(position);

    eval
}

pub use material::ROLE_VALUE;
