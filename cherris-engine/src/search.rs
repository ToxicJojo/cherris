use std::{
    sync::{Arc, Mutex},
    thread,
};

use cherris_core::{
    Color, Move, Position,
    uci::{UCIGoParams, UCIGuiCommand},
};

use crate::{iterative_deepening::iterative_deepening, time_managment::TimeManagment};

use self::transposition_table::TranspositionTable;

pub mod alpha_beta;
pub mod history;
pub mod iterative_deepening;
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
    pub current_depth: u8,
    pub selective_depth: u8,
}

pub struct Search {}

impl Search {
    pub fn run(
        position: Position,
        search_params: UCIGoParams,
        transposition_table: Arc<Mutex<TranspositionTable>>,
    ) {
        thread::spawn(move || {
            let max_depth = search_params.depth.unwrap_or(DEFAULT_MAX_DEPTH);
            let max_nodes = search_params.nodes.unwrap_or(u64::MAX);

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
            let best_move = iterative_deepening(
                position,
                max_depth,
                max_nodes,
                time_managment,
                transposition_table,
            );

            let best_move = match (best_move, position.color_to_move) {
                (Move::CastleShort, Color::White) => "e1g1".to_string(),
                (Move::CastleShort, Color::Black) => "e8g8".to_string(),
                (Move::CastleLong, Color::White) => "e1c1".to_string(),
                (Move::CastleLong, Color::Black) => "e8c8".to_string(),
                _ => best_move.to_string(),
            };

            print!("{}", UCIGuiCommand::BestMove(best_move));
        });
    }
}
