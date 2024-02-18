use std::i16;

use cherris_core::{Color, Move, Square};

pub struct HistoryTable([[[i16; Square::COUNT]; Square::COUNT]; Color::COUNT]);

impl HistoryTable {
    pub fn new() -> HistoryTable {
        HistoryTable([[[0; Square::COUNT]; Square::COUNT]; Color::COUNT])
    }

    pub fn get(&self, color: Color, chess_move: Move) -> i16 {
        match chess_move {
            Move::Standard { from, to, .. } => self.0[color][from][to],
            Move::EnPassant { from, to, .. } => self.0[color][from][to],
            _ => 0,
        }
    }

    pub fn update(&mut self, color: Color, chess_move: Move, depth: u8) {
        if let Move::Standard {
            from,
            to,
            capture: None,
            ..
        } = chess_move
        {
            self.0[color][from][to] += (depth * depth) as i16
        }
    }
}

impl Default for HistoryTable {
    fn default() -> Self {
        Self::new()
    }
}
