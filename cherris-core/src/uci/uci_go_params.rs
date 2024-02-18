use std::{fmt::Display, u128};

use crate::uci_engine_command::UCIEngineCommandParseError;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct UCIGoParams {
    pub search_moves: Vec<String>,
    pub ponder: bool,
    pub w_time: Option<u128>,
    pub b_time: Option<u128>,
    pub w_inc: Option<u128>,
    pub b_inc: Option<u128>,
    pub moves_to_go: Option<u64>,
    pub depth: Option<u8>,
    pub nodes: Option<u64>,
    pub mate: Option<u64>,
    pub movetime: Option<u64>,
    pub infinite: bool,
}

impl UCIGoParams {
    pub fn from_tokens(tokens: Vec<&str>) -> Result<UCIGoParams, UCIEngineCommandParseError> {
        enum Mode {
            Empty,
            SearchMoves,
            WTime,
            BTime,
            BInc,
            WInc,
            MovesToGo,
            Depth,
            Mate,
            Nodes,
            MoveTime,
        }

        let mut mode = Mode::Empty;
        let mut search_params = UCIGoParams::default();

        for token in tokens {
            match token {
                "go" => continue,
                "searchmoves" => mode = Mode::SearchMoves,
                "ponder" => search_params.ponder = true,
                "wtime" => mode = Mode::WTime,
                "btime" => mode = Mode::BTime,
                "winc" => mode = Mode::WInc,
                "binc" => mode = Mode::BInc,
                "movestogo" => mode = Mode::MovesToGo,
                "depth" => mode = Mode::Depth,
                "nodes" => mode = Mode::Nodes,
                "mate" => mode = Mode::Mate,
                "movetime" => mode = Mode::MoveTime,
                "infinite" => search_params.infinite = true,
                _ => match mode {
                    Mode::WTime => search_params.w_time = Some(token.parse()?),
                    Mode::BTime => search_params.b_time = Some(token.parse()?),
                    Mode::WInc => search_params.w_inc = Some(token.parse()?),
                    Mode::BInc => search_params.b_inc = Some(token.parse()?),
                    Mode::MovesToGo => search_params.moves_to_go = Some(token.parse()?),
                    Mode::Depth => search_params.depth = Some(token.parse()?),
                    Mode::Nodes => search_params.nodes = Some(token.parse()?),
                    Mode::Mate => search_params.mate = Some(token.parse()?),
                    Mode::MoveTime => search_params.movetime = Some(token.parse()?),
                    Mode::SearchMoves => search_params.search_moves.push(token.to_string()),
                    Mode::Empty => (),
                },
            }
        }

        Ok(search_params)
    }
}

impl Display for UCIGoParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.search_moves.is_empty() {
            write!(f, " searchmoves {}", self.search_moves.join(" "))?;
        }

        if self.ponder {
            write!(f, " ponder")?;
        }

        if let Some(w_time) = self.w_time {
            write!(f, " wtime {}", w_time)?;
        }

        if let Some(b_time) = self.b_time {
            write!(f, " btime {}", b_time)?;
        }

        if let Some(w_inc) = self.w_inc {
            write!(f, " winc {}", w_inc)?;
        }

        if let Some(b_inc) = self.b_inc {
            write!(f, " binc {}", b_inc)?;
        }

        if let Some(moves_to_go) = self.moves_to_go {
            write!(f, " movestogo {}", moves_to_go)?;
        }

        if let Some(depth) = self.depth {
            write!(f, " depth {}", depth)?;
        }

        if let Some(nodes) = self.nodes {
            write!(f, " nodes {}", nodes)?;
        }

        if let Some(mate) = self.mate {
            write!(f, " mate {}", mate)?;
        }

        if let Some(movetime) = self.movetime {
            write!(f, " movetime {}", movetime)?;
        }

        if self.infinite {
            write!(f, " infinite")?;
        }

        Ok(())
    }
}
