use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use cherris_core::{
    generate_lookup_tables,
    uci::{UCIEngineCommand, UCIGoParams, UCIGuiCommand},
    Move, Position, LAN,
};

use crate::{transposition_table::TranspositionTable, Search};

pub struct Engine {
    position: Position,
    uci_search_params: UCIGoParams,
    transposition_table: Arc<Mutex<TranspositionTable>>,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            position: Position::default(),
            uci_search_params: UCIGoParams::default(),
            transposition_table: Arc::new(Mutex::new(TranspositionTable::new(2_u64.pow(24)))),
        }
    }

    pub fn run(&mut self) {
        let stdin = std::io::stdin();
        let mut input = String::new();

        loop {
            stdin
                .read_line(&mut input)
                .expect("Failed to read from stdin");

            let command = UCIEngineCommand::from_str(&input);
            if let Ok(command) = command {
                match command {
                    UCIEngineCommand::Uci => {
                        generate_lookup_tables();
                        self.send_command(UCIGuiCommand::IdName("cherris".to_string()));
                        self.send_command(UCIGuiCommand::IdAuthor("Johannes Thiel".to_string()));
                        self.send_command(UCIGuiCommand::UciOk);
                    }
                    UCIEngineCommand::Position(params) => {
                        self.position = Position::from_str(&params.fen).unwrap();
                        for mv in params.moves {
                            if let Ok(lan) = LAN::from_str(&mv) {
                                if let Ok(mv) = Move::from_lan(&lan, &self.position) {
                                    self.position.make_move(mv);
                                }
                            }
                        }
                    }
                    UCIEngineCommand::Go(search_params) => {
                        self.uci_search_params = search_params;
                        Search::run(
                            self.position,
                            self.uci_search_params.clone(),
                            self.transposition_table.clone(),
                        );
                    }
                    UCIEngineCommand::IsReady => self.send_command(UCIGuiCommand::ReadyOk),
                    UCIEngineCommand::Quit => break,
                    _ => {}
                }
            }

            input.clear();
        }
    }

    fn send_command(&self, command: UCIGuiCommand) {
        print!("{}", command);
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}
