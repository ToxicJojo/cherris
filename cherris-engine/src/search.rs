use std::{
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

use cherris_core::{Color, Move, Position};

use crate::{time_managment::TimeManagment, UCIGuiCommand, UCISearchInfo, UCISearchParams};

use self::{alpha_beta::alpha_beta, transposition_table::TranspositionTable};

pub mod alpha_beta;
pub mod move_sort;
pub mod quiescence;
pub mod time_managment;
pub mod transposition_table;

const DEFAULT_MAX_DEPTH: u8 = 10;

pub struct SearchData {
    pub nodes: u64,
    pub max_nodes: u64,
    pub pv: Vec<Move>,
    pub transposition_table: Arc<Mutex<TranspositionTable>>,
}

pub struct Search {}

impl Search {
    pub fn run(
        position: Position,
        search_params: UCISearchParams,
        transposition_table: Arc<Mutex<TranspositionTable>>,
    ) {
        thread::spawn(move || {
            let mut depth = 1;

            let max_depth = search_params.depth.unwrap_or(DEFAULT_MAX_DEPTH);

            let mut pv = Vec::with_capacity(max_depth.into());

            let (time, increment) = match position.color_to_move {
                Color::White => (
                    search_params.w_time.unwrap_or(u64::MAX),
                    search_params.w_inc.unwrap_or_default(),
                ),
                Color::Black => (
                    search_params.b_time.unwrap_or(u64::MAX),
                    search_params.b_inc.unwrap_or_default(),
                ),
            };

            let time_managment = TimeManagment::new(time, increment);

            while depth <= max_depth {
                let timer = Instant::now();
                let mut search_data = SearchData {
                    nodes: 0,
                    max_nodes: search_params.nodes.unwrap_or(u64::MAX),
                    pv: pv.clone(),
                    transposition_table: transposition_table.clone(),
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

                if eval <= i16::MIN + 6 || eval >= i16::MAX - 6 {
                    break;
                }

                if search_data.nodes > search_data.max_nodes {
                    break;
                }

                if !time_managment.has_time_for_next_iteration(timer.elapsed().as_millis()) {
                    break;
                }

                depth += 1;
            }

            print!("{}", UCIGuiCommand::BestMove(pv[0].to_string()));
        });
    }
}
