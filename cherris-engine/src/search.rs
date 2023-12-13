use std::thread;

use cherris_core::{Color, Move, Position};

use crate::{alpha_beta::alpha_beta_min, UCIGuiCommand, UCISearchInfo, UCISearchParams};

use self::alpha_beta::alpha_beta_max;

pub mod alpha_beta;

pub struct SearchData {
    pub nodes: u64,
    pub pv: Vec<Move>,
}

pub struct Search {}

impl Search {
    pub fn run(position: Position, _search_params: &UCISearchParams) {
        thread::spawn(move || {
            let mut depth = 1;

            let mut pv = Vec::new();
            while depth <= 3 {
                let mut search_data = SearchData {
                    nodes: 0,
                    pv: pv.clone(),
                };

                pv.clear();

                let eval = match position.color_to_move {
                    Color::White => alpha_beta_max(
                        i16::MIN,
                        i16::MAX,
                        depth,
                        &mut pv,
                        &position,
                        &mut search_data,
                    ),
                    Color::Black => alpha_beta_min(
                        i16::MIN,
                        i16::MAX,
                        depth,
                        &mut pv,
                        &position,
                        &mut search_data,
                    ),
                };

                let search_info = UCISearchInfo {
                    depth,
                    seldepth: depth,
                    time: 0,
                    score: eval,
                    nodes: search_data.nodes,
                    pv: pv.clone(),
                };
                let info_command = UCIGuiCommand::Info(search_info);
                print!("{}", info_command);

                if eval == i16::MIN + 1 || eval == i16::MAX - 1 {
                    break;
                }

                depth += 1;
            }

            print!("{}", UCIGuiCommand::BestMove(pv[0].to_string()));
        });
    }
}
