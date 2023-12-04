use std::{fmt::Display, str::FromStr};

use cherris_core::{Move, Position};

/// Represents all UCI commands that can be sent to an engine.
#[derive(Debug, PartialEq)]
pub enum UCIEngineCommand {
    Uci,
    Debug(bool),
    IsReady,
    SetOption,
    Register,
    UciNewGame,
    Position(Position, Vec<Move>),
    Go,
    Stop,
    PonderHit,
    Quit,
}

#[derive(Debug, PartialEq)]
pub enum UCIEngineCommandParseError {
    InvalidCommand,
}

impl FromStr for UCIEngineCommand {
    type Err = UCIEngineCommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command = s.trim();
        let parts: Vec<&str> = command.split(" ").collect();

        match parts[0] {
            "uci" => Ok(UCIEngineCommand::Uci),
            "isready" => Ok(UCIEngineCommand::IsReady),
            "ucinewgame" => Ok(UCIEngineCommand::UciNewGame),
            "stop" => Ok(UCIEngineCommand::Stop),
            "ponderhit" => Ok(UCIEngineCommand::PonderHit),
            "quit" => Ok(UCIEngineCommand::Quit),
            "position" => {
                let position = match parts[1] {
                    "startpos" => Position::default(),
                    "fen" => {
                        let fen: Vec<String> = parts
                            .iter()
                            .skip(2)
                            .take(6)
                            .map(|&s| s.to_owned())
                            .collect();
                        Position::from_str(&fen.join(" ")).unwrap()
                    }
                    _ => return Err(UCIEngineCommandParseError::InvalidCommand),
                };

                Ok(UCIEngineCommand::Position(position, Vec::new()))
            }
            _ => Err(UCIEngineCommandParseError::InvalidCommand),
        }
    }
}

/// Represents all UCI commands that can be sent to a GUI.
pub enum UCIGuiCommand {
    IdName(String),
    IdAuthor(String),
    UciOk,
    ReadyOk,
    BestMove,
    CopyProtection,
    Registration,
    Info,
    Option(EngineOption),
}

pub struct EngineOption {
    pub id: String,
    pub r#type: EngineOptionType,
    pub default: Option<String>,
    pub min: Option<String>,
    pub max: Option<String>,
}

pub enum EngineOptionType {
    Check,
    Spin,
    Combo,
    Button,
    String,
}

impl Display for EngineOptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineOptionType::Check => write!(f, "check"),
            EngineOptionType::Spin => write!(f, "spin"),
            EngineOptionType::Combo => write!(f, "combo"),
            EngineOptionType::Button => write!(f, "button"),
            EngineOptionType::String => write!(f, "string"),
        }
    }
}

impl Display for UCIGuiCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UCIGuiCommand::IdName(name) => writeln!(f, "id name {}", name),
            UCIGuiCommand::IdAuthor(author) => writeln!(f, "id author {}", author),
            UCIGuiCommand::UciOk => writeln!(f, "uciok"),
            UCIGuiCommand::ReadyOk => writeln!(f, "readyok"),
            UCIGuiCommand::Option(option) => {
                write!(f, "option name {} type {}", option.id, option.r#type)?;
                if let Some(default) = &option.default {
                    write!(f, " default {}", default)?;
                }

                if let Some(min) = &option.min {
                    write!(f, " min {}", min)?;
                }

                if let Some(max) = &option.max {
                    write!(f, " max {}", max)?;
                }

                write!(f, "\n")
            }
            _ => writeln!(f, ""),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_engine_uci() {
        let command = UCIEngineCommand::from_str("uci").unwrap();

        assert_eq!(command, UCIEngineCommand::Uci);
    }

    #[test]
    fn parse_engine_isready() {
        let command = UCIEngineCommand::from_str("isready").unwrap();

        assert_eq!(command, UCIEngineCommand::IsReady);
    }

    #[test]
    fn parse_engine_uci_new_game() {
        let command = UCIEngineCommand::from_str("ucinewgame").unwrap();

        assert_eq!(command, UCIEngineCommand::UciNewGame);
    }

    #[test]
    fn parse_engine_stop() {
        let command = UCIEngineCommand::from_str("stop").unwrap();

        assert_eq!(command, UCIEngineCommand::Stop);
    }

    #[test]
    fn parse_engine_ponder_hit() {
        let command = UCIEngineCommand::from_str("ponderhit").unwrap();

        assert_eq!(command, UCIEngineCommand::PonderHit);
    }

    #[test]
    fn parse_engine_quit() {
        let command = UCIEngineCommand::from_str("quit").unwrap();

        assert_eq!(command, UCIEngineCommand::Quit);
    }

    #[test]
    fn parse_engine_position_startpos() {
        let command = UCIEngineCommand::from_str("position startpos").unwrap();

        assert_eq!(
            command,
            UCIEngineCommand::Position(Position::default(), Vec::new())
        );
    }

    #[test]
    fn parse_engine_position_fen() {
        let command = UCIEngineCommand::from_str(
            "position fen rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1",
        )
        .unwrap();

        assert_eq!(
            command,
            UCIEngineCommand::Position(
                Position::from_str("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1")
                    .unwrap(),
                Vec::new()
            )
        );
    }

    #[test]
    fn format_gui_id_name() {
        let command = UCIGuiCommand::IdName("cherris".to_string());

        assert_eq!(command.to_string(), "id name cherris\n");
    }

    #[test]
    fn format_gui_id_author() {
        let command = UCIGuiCommand::IdAuthor("peter".to_string());

        assert_eq!(command.to_string(), "id author peter\n");
    }

    #[test]
    fn format_gui_uciok() {
        let command = UCIGuiCommand::UciOk;

        assert_eq!(command.to_string(), "uciok\n");
    }

    #[test]
    fn format_gui_ready_ok() {
        let command = UCIGuiCommand::ReadyOk;

        assert_eq!(command.to_string(), "readyok\n");
    }

    #[test]
    fn format_gui_option() {
        let command = UCIGuiCommand::Option(EngineOption {
            id: "Hash".to_string(),
            r#type: EngineOptionType::Spin,
            default: Some("16".to_string()),
            min: Some("1".to_string()),
            max: Some("256".to_string()),
        });

        assert_eq!(
            command.to_string(),
            "option name Hash type spin default 16 min 1 max 256\n"
        );
    }
}
