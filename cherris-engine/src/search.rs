use std::{thread, time::Instant};

use cherris_core::{Move, Position};

use crate::{UCIGuiCommand, UCISearchInfo, UCISearchParams};

use self::{alpha_beta::alpha_beta, transposition_table::TranspositionTable};

pub mod alpha_beta;
pub mod transposition_table;

const DEFAULT_MAX_DEPTH: u8 = 5;

pub struct SearchData {
    pub nodes: u64,
    pub pv: Vec<Move>,
    pub transposition_table: TranspositionTable,
}

pub struct Search {}

impl Search {
    pub fn run(position: Position, search_params: UCISearchParams) {
        thread::spawn(move || {
            let mut depth = 1;

            let max_depth = search_params.depth.unwrap_or(DEFAULT_MAX_DEPTH);

            let mut pv = Vec::new();
            while depth <= max_depth {
                let timer = Instant::now();
                let mut search_data = SearchData {
                    nodes: 0,
                    pv: pv.clone(),
                    transposition_table: TranspositionTable::new(2_u64.pow(16)),
                };

                pv.clear();

                let eval = alpha_beta(
                    i16::MIN + 1,
                    i16::MAX - 1,
                    depth,
                    &mut pv,
                    &position,
                    &mut search_data,
                );

                let search_info = UCISearchInfo {
                    depth,
                    seldepth: depth,
                    time: timer.elapsed().as_millis(),
                    score: eval,
                    nodes: search_data.nodes,
                    pv: pv.clone(),
                };
                let info_command = UCIGuiCommand::Info(search_info);
                print!("{}", info_command);

                if eval == i16::MIN + 2 || eval == i16::MAX - 2 {
                    break;
                }

                depth += 1;
            }

            print!("{}", UCIGuiCommand::BestMove(pv[0].to_string()));
        });
    }
}
