use std::{
    sync::{Arc, Mutex},
    thread,
    time::Instant,
    u128,
};

use cherris_core::{
    uci::{UCIGoParams, UCIGuiCommand, UCISearchInfo},
    Color, Move, Position,
};

use crate::time_managment::TimeManagment;

use self::{
    alpha_beta::alpha_beta, history::HistoryTable, transposition_table::TranspositionTable,
};

pub mod alpha_beta;
pub mod history;
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
    pub history_table: Arc<Mutex<HistoryTable>>,
}

pub struct Search {}

impl Search {
    pub fn run(
        position: Position,
        search_params: UCIGoParams,
        transposition_table: Arc<Mutex<TranspositionTable>>,
    ) {
        thread::spawn(move || {
            let mut depth = 1;

            let max_depth = search_params.depth.unwrap_or(DEFAULT_MAX_DEPTH);

            let mut pv = Vec::with_capacity(max_depth.into());

            let (time, increment) = match position.color_to_move {
                Color::White => (
                    search_params.w_time.unwrap_or(u128::MAX),
                    search_params.w_inc.unwrap_or_default(),
                ),
                Color::Black => (
                    search_params.b_time.unwrap_or(u128::MAX),
                    search_params.b_inc.unwrap_or_default(),
                ),
            };

            let time_managment = TimeManagment::new(time, increment, search_params.moves_to_go);
            let history_table = Arc::new(Mutex::new(HistoryTable::new()));

            while depth <= max_depth {
                let timer = Instant::now();
                let mut search_data = SearchData {
                    nodes: 0,
                    max_nodes: search_params.nodes.unwrap_or(u64::MAX),
                    pv: pv.clone(),
                    transposition_table: transposition_table.clone(),
                    history_table: history_table.clone(),
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

                let elapsed = timer.elapsed().as_millis().max(1);
                let nps = ((search_data.nodes as u128) / elapsed) as u64 * 1000;

                let search_info = UCISearchInfo {
                    depth,
                    seldepth: depth,
                    time: elapsed,
                    score: eval,
                    nodes: search_data.nodes,
                    pv: pv.iter().map(|mv| mv.to_string()).collect(),
                    nps,
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

            let best_move = match (pv[0], position.color_to_move) {
                (Move::CastleShort, Color::White) => "e1g1".to_string(),
                (Move::CastleShort, Color::Black) => "e8g8".to_string(),
                (Move::CastleLong, Color::White) => "e1c1".to_string(),
                (Move::CastleLong, Color::Black) => "e8c8".to_string(),
                _ => pv[0].to_string(),
            };

            print!("{}", UCIGuiCommand::BestMove(best_move));
        });
    }
}
