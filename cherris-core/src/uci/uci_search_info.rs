use std::fmt::Display;

#[derive(Clone)]
pub enum UCIScore {
    Centipawns(i16),
    Mate(i16),
}

#[derive(Clone)]
pub struct UCISearchInfo {
    pub depth: u8,
    pub seldepth: u8,
    pub time: u128,
    pub nodes: u64,
    pub score: UCIScore,
    pub pv: Vec<String>,
    pub nps: u64,
}

impl Display for UCISearchInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "depth {} seldepth {} score {} time {} nodes {} nps {} pv",
            self.depth, self.seldepth, self.score, self.time, self.nodes, self.nps
        )?;

        for mv in &self.pv {
            write!(f, " {}", mv)?;
        }

        Ok(())
    }
}

impl Display for UCIScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UCIScore::Centipawns(score) => write!(f, "cp {}", score),
            UCIScore::Mate(mate) => write!(f, "mate {}", mate),
        }
    }
}
