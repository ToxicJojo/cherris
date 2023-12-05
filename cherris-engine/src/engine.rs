use std::str::FromStr;

use cherris_core::{generate_lookup_tables, Position};

use crate::{UCIEngineCommand, UCIGuiCommand};

pub struct Engine {
    position: Position,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            position: Position::default(),
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
                    UCIEngineCommand::Position(position, moves) => {
                        self.position = position;
                        for mv in moves {
                            self.position.make_move(mv)
                        }
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
