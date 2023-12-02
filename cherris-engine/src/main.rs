use std::str::FromStr;

use cherris_core::{generate_lookup_tables, Position};
use cherris_engine::alpha_beta_max;

fn main() {
    generate_lookup_tables();
    let mut pos = Position::from_str("8/3q1k2/8/8/2N5/8/8/1K6 w - - 0 1").unwrap();

    let score = alpha_beta_max(f32::MIN, f32::MAX, 6, &mut pos);
    println!("Eval: {}", score);
}
