use cherris_core::{Move, Zobrist};

#[derive(Clone, Copy, Debug)]
pub struct TranspositionEntry {
    pub zobrist: Zobrist,
    pub score: i16,
    pub depth: u8,
    pub entry_type: TranspositionEntryType,
    pub chess_move: Move,
}

#[derive(Clone, Copy, Debug)]
pub enum TranspositionEntryType {
    Exact,
    UpperBound,
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

    pub fn get(&self, zobrist: Zobrist) -> &Option<TranspositionEntry> {
        let key = ((zobrist.key() % self.size) & zobrist.key()) as usize;

        self.entries.get(key).unwrap()
    }

    pub fn insert(&mut self, entry: TranspositionEntry) {
        let key = ((entry.zobrist.key() % self.size) & entry.zobrist.key()) as usize;

        self.entries[key] = Some(entry);
    }
}
