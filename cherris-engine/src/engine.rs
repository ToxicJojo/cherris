use std::str::FromStr;

use cherris_core::{generate_lookup_tables, Move, Position, LAN};

use crate::{Search, UCIEngineCommand, UCIGuiCommand, UCISearchParams};

pub struct Engine {
    position: Position,
    uci_search_params: UCISearchParams,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            position: Position::default(),
            uci_search_params: UCISearchParams::default(),
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
                    UCIEngineCommand::Position(fen, moves) => {
                        self.position = Position::from_str(&fen).unwrap();
                        for mv in moves {
                            if let Ok(lan) = LAN::from_str(&mv) {
                                if let Ok(mv) = Move::from_lan(&lan, &self.position) {
                                    self.position.make_move(mv);
                                }
                            }
                        }
                    }
                    UCIEngineCommand::Go(search_params) => {
                        self.uci_search_params = search_params;
                        Search::run(self.position, self.uci_search_params.clone());
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
