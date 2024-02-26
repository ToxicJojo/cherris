use cherris_core::{Move, Position, Zobrist};

use crate::evaluation::Evaluation;

// An entry in the transposition table.
#[derive(Clone, Copy, Debug)]
pub struct TranspositionEntry {
    pub zobrist: Zobrist,
    pub score: Evaluation,
    pub depth: u8,
    pub entry_type: TranspositionEntryType,
    pub chess_move: Move,
}

/// The type of entry in the transposition table.
#[derive(Clone, Copy, Debug)]
pub enum TranspositionEntryType {
    /// The score of this entry is exact as it caused an alpha cutoff
    Exact,
    /// The score of this entry is only an upperbound
    UpperBound,
    /// The score of this entry is only a lower bound as it caused an beta cutoff
    LowerBound,
}

#[derive(Clone, Debug)]
pub struct TranspositionTable {
    entries: Vec<Option<TranspositionEntry>>,
    size: u64,
}

impl TranspositionTable {
    pub fn new(size: u64) -> TranspositionTable {
        TranspositionTable {
            entries: vec![None; size as usize],
            size,
        }
    }

    fn get_entry_index(&self, zobrist: Zobrist) -> usize {
        ((zobrist.key() % self.size) & zobrist.key()) as usize
    }

    pub fn get_entry(&self, zobrist: Zobrist) -> &Option<TranspositionEntry> {
        let key = self.get_entry_index(zobrist);

        self.entries.get(key).unwrap()
    }

    pub fn insert(&mut self, entry: TranspositionEntry) {
        let key = self.get_entry_index(entry.zobrist);

        self.entries[key] = Some(entry);
    }

    pub fn get(
        &self,
        position: &Position,
        alpha: Evaluation,
        beta: Evaluation,
        depth: u8,
    ) -> (Option<Move>, Option<Evaluation>) {
        let zobrist = position.zobrist;
        let key = self.get_entry_index(zobrist);

        let entry = unsafe { self.entries.get_unchecked(key) };
        let mut tt_move = None;
        let mut tt_value = None;

        if let Some(entry) = entry {
            if entry.zobrist == zobrist {
                tt_move = Some(entry.chess_move);
                if entry.depth >= depth {
                    match entry.entry_type {
                        TranspositionEntryType::Exact => tt_value = Some(entry.score),
                        TranspositionEntryType::UpperBound => {
                            if entry.score <= alpha {
                                tt_value = Some(entry.score)
                            }
                        }
                        TranspositionEntryType::LowerBound => {
                            if entry.score >= beta {
                                tt_value = Some(entry.score)
                            }
                        }
                    }
                }
            }
        }

        (tt_move, tt_value)
    }
}
