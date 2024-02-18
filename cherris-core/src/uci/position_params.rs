use std::fmt::Display;

use crate::{uci_engine_command::UCIEngineCommandParseError, Position};

#[derive(Debug, PartialEq)]
pub struct UCIPositionParams {
    pub fen: String,
    pub moves: Vec<String>,
}

impl UCIPositionParams {
    pub fn startpos(moves: Vec<String>) -> UCIPositionParams {
        UCIPositionParams {
            fen: Position::STARTING_FEN.to_string(),
            moves,
        }
    }

    pub fn from_token(tokens: Vec<&str>) -> Result<UCIPositionParams, UCIEngineCommandParseError> {
        let mut reading_fen = false;
        let mut reading_moves = false;
        let mut fen = Vec::new();
        let mut moves = Vec::new();

        for token in tokens {
            match token {
                "position" => continue,
                "startpos" => continue,
                "fen" => reading_fen = true,
                "moves" => {
                    reading_moves = true;
                    reading_fen = false
                }
                _ => {
                    if reading_fen {
                        fen.push(token);
                    } else if reading_moves {
                        moves.push(token.to_string());
                    }
                }
            }
        }

        let mut fen = fen.join(" ");
        if fen.is_empty() {
            fen = Position::STARTING_FEN.to_string();
        } else {
            fen = fen.trim().to_string();
        }

        Ok(UCIPositionParams { fen, moves })
    }
}

impl Display for UCIPositionParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.fen == Position::STARTING_FEN {
            write!(f, "startpos")?;
        } else {
            write!(f, "fen {}", self.fen)?;
        }

        if !self.moves.is_empty() {
            write!(f, " moves {}", self.moves.join(" "))?;
        }

        Ok(())
    }
}
