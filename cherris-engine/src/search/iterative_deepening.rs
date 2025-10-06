use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use cherris_core::{
    Move, Position,
    uci::{UCIGuiCommand, UCISearchInfo},
};

use crate::{
    SearchData, alpha_beta::alpha_beta, evaluation::Evaluation, time_managment::TimeManagment,
    transposition_table::TranspositionTable,
};

pub fn iterative_deepening(
    position: Position,
    max_depth: u8,
    max_nodes: u64,
    time_managment: TimeManagment,
    transposition_table: Arc<Mutex<TranspositionTable>>,
) -> Move {
    let mut depth = 1;
    let mut pv = Vec::with_capacity(max_depth.into());

    while depth <= max_depth {
        let timer = Instant::now();
        let mut search_data = SearchData {
            nodes: 0,
            max_nodes,
            pv: pv.clone(),
            transposition_table: transposition_table.clone(),
            current_depth: 0,
            selective_depth: 0,
        };

        pv.clear();

        let eval = alpha_beta(
            Evaluation::MIN,
            Evaluation::MAX,
            depth,
            &mut pv,
            &position,
            &mut search_data,
        );

        let elapsed = timer.elapsed().as_millis().max(1);
        let nps = ((search_data.nodes as u128) / elapsed) as u64 * 1000;

        let search_info = UCISearchInfo {
            depth,
            seldepth: search_data.selective_depth,
            time: elapsed,
            score: eval.into(),
            nodes: search_data.nodes,
            pv: pv.iter().map(|mv| mv.to_string()).collect(),
            nps,
        };
        let info_command = UCIGuiCommand::Info(search_info);
        if !cfg!(test) {
            print!("{}", info_command);
        }

        if eval.is_checkmate() {
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

    pv[0]
}
