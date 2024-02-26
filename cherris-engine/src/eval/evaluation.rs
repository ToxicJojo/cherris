use core::f64;
use std::{
    i16,
    ops::{AddAssign, Neg},
    u8,
};

use cherris_core::uci::UCIScore;

const CHECKMATE_BASE: i16 = -32000;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Evaluation(i16);

impl Evaluation {
    pub const DRAW: Evaluation = Evaluation(0);

    pub const CHECKMATE: Evaluation = Evaluation(CHECKMATE_BASE);

    pub const MIN: Evaluation = Evaluation(i16::MIN + 1);
    pub const MAX: Evaluation = Evaluation(i16::MAX - 1);

    pub fn new(eval: i16) -> Evaluation {
        Evaluation(eval)
    }

    pub fn new_mate_in(depth: u8) -> Evaluation {
        Evaluation(CHECKMATE_BASE + depth as i16)
    }

    pub fn is_checkmate(&self) -> bool {
        self.0 >= (-CHECKMATE_BASE - 100) || self.0 <= (CHECKMATE_BASE + 100)
    }

    pub fn centipawns(&self) -> i16 {
        self.0
    }

    pub fn mate_in(&self) -> i16 {
        self.0.abs() + CHECKMATE_BASE
    }
}

impl AddAssign for Evaluation {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Neg for Evaluation {
    type Output = Evaluation;

    fn neg(self) -> Self::Output {
        Evaluation(-self.0)
    }
}

impl From<Evaluation> for UCIScore {
    fn from(value: Evaluation) -> Self {
        if value.is_checkmate() {
            UCIScore::Mate((value.mate_in() as f64 / 2.0).ceil() as i16)
        } else {
            UCIScore::Centipawns(value.centipawns())
        }
    }
}
