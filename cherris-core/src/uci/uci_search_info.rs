use std::fmt::Display;

pub struct UCISearchInfo {
    pub depth: u8,
    pub seldepth: u8,
    pub time: u128,
    pub nodes: u64,
    pub score: i16,
    pub pv: Vec<String>,
    pub nps: u64,
}

impl Display for UCISearchInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "depth {} seldepth {} score cp {} time {} nodes {} nps {} pv",
            self.depth, self.seldepth, self.score, self.time, self.nodes, self.nps
        )?;

        for mv in &self.pv {
            write!(f, " {}", mv)?;
        }

        Ok(())
    }
}
