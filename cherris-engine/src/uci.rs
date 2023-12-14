use std::{fmt::Display, str::FromStr};

use cherris_core::{Move, Position};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct UCISearchParams {
    pub search_moves: Vec<String>,
    pub ponder: bool,
    pub w_time: Option<u64>,
    pub b_time: Option<u64>,
    pub w_inc: Option<u64>,
    pub b_inc: Option<u64>,
    pub moves_to_go: Option<u64>,
    pub depth: Option<u8>,
    pub nodes: Option<u64>,
    pub mate: Option<u64>,
    pub movetime: Option<u64>,
    pub infinite: bool,
}

pub struct UCISearchInfo {
    pub depth: u8,
    pub seldepth: u8,
    pub time: u128,
    pub nodes: u64,
    pub score: i16,
    pub pv: Vec<Move>,
}

#[derive(Debug, PartialEq)]
pub enum UCIEngineCommandParseError {
    InvalidCommand,
}

/// Represents all UCI commands that can be sent to an engine.
#[derive(Debug, PartialEq)]
pub enum UCIEngineCommand {
    Uci,
    Debug(bool),
    IsReady,
    SetOption,
    Register,
    UciNewGame,
    Position(String, Vec<String>),
    Go(UCISearchParams),
    Stop,
    PonderHit,
    Quit,
}

impl UCIEngineCommand {
    fn parse_position(parts: Vec<&str>) -> Result<UCIEngineCommand, UCIEngineCommandParseError> {
        let mut reading_fen = false;
        let mut reading_moves = false;
        let mut fen = String::new();
        let mut moves = Vec::new();

        for part in parts {
            match part {
                "position" => continue,
                "startpos" => continue,
                "fen" => reading_fen = true,
                "moves" => {
                    reading_moves = true;
                    reading_fen = false
                }
                _ => {
                    if reading_fen {
                        fen.push_str(part);
                        fen.push(' ');
                    } else if reading_moves {
                        moves.push(part.to_string());
                    }
                }
            }
        }

        if fen.is_empty() {
            fen = Position::STARTING_FEN.to_string();
        } else {
            fen = fen.trim().to_string();
        }

        Ok(UCIEngineCommand::Position(fen, moves))
    }

    fn parse_go(parts: Vec<&str>) -> Result<UCIEngineCommand, UCIEngineCommandParseError> {
        enum Token {
            Empty,
            SearchMoves,
            WTime,
            BTime,
            BInc,
            WInc,
            MovesToGo,
            Depth,
            Mate,
            Nodes,
            MoveTime,
        }

        let mut token = Token::Empty;
        let mut search_params = UCISearchParams::default();

        for part in parts {
            match part {
                "go" => continue,
                "searchmoves" => token = Token::SearchMoves,
                "ponder" => search_params.ponder = true,
                "wtime" => token = Token::WTime,
                "btime" => token = Token::BTime,
                "winc" => token = Token::WInc,
                "binc" => token = Token::BInc,
                "movestogo" => token = Token::MovesToGo,
                "depth" => token = Token::Depth,
                "nodes" => token = Token::Nodes,
                "mate" => token = Token::Mate,
                "movetime" => token = Token::MoveTime,
                "infinite" => search_params.infinite = true,
                _ => match token {
                    Token::WTime => search_params.w_time = Some(u64::from_str(part).unwrap()),
                    Token::BTime => search_params.b_time = Some(u64::from_str(part).unwrap()),
                    Token::WInc => search_params.w_inc = Some(u64::from_str(part).unwrap()),
                    Token::BInc => search_params.b_inc = Some(u64::from_str(part).unwrap()),
                    Token::MovesToGo => {
                        search_params.moves_to_go = Some(u64::from_str(part).unwrap())
                    }
                    Token::Depth => search_params.depth = Some(u8::from_str(part).unwrap()),
                    Token::Nodes => search_params.nodes = Some(u64::from_str(part).unwrap()),
                    Token::Mate => search_params.mate = Some(u64::from_str(part).unwrap()),
                    Token::MoveTime => search_params.movetime = Some(u64::from_str(part).unwrap()),
                    Token::SearchMoves => search_params.search_moves.push(part.to_string()),
                    Token::Empty => (),
                },
            }
        }

        Ok(UCIEngineCommand::Go(search_params))
    }
}

impl FromStr for UCIEngineCommand {
    type Err = UCIEngineCommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command = s.trim();
        let parts: Vec<&str> = command.split(' ').collect();

        match parts[0] {
            "uci" => Ok(UCIEngineCommand::Uci),
            "isready" => Ok(UCIEngineCommand::IsReady),
            "ucinewgame" => Ok(UCIEngineCommand::UciNewGame),
            "ponderhit" => Ok(UCIEngineCommand::PonderHit),
            "position" => UCIEngineCommand::parse_position(parts),
            "go" => UCIEngineCommand::parse_go(parts),
            "stop" => Ok(UCIEngineCommand::Stop),
            "quit" => Ok(UCIEngineCommand::Quit),
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
    BestMove(String),
    CopyProtection,
    Registration,
    Info(UCISearchInfo),
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
            UCIGuiCommand::BestMove(mv) => writeln!(f, "bestmove {}", mv),
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

                writeln!(f)
            }
            UCIGuiCommand::Info(info) => {
                write!(
                    f,
                    "info depth {} seldepth {} score cp {} time {} nodes {} pv",
                    info.depth, info.seldepth, info.score, info.time, info.nodes
                )?;

                for mv in info.pv.clone() {
                    write!(f, " {}", mv)?;
                }

                writeln!(f)
            }
            _ => writeln!(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

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
        let command = UCIEngineCommand::from_str("position startpos moves e2e4").unwrap();

        assert_eq!(
            command,
            UCIEngineCommand::Position(
                Position::STARTING_FEN.to_string(),
                vec!["e2e4".to_string()]
            )
        );
    }

    #[test]
    fn parse_engine_position_fen() {
        let command = UCIEngineCommand::from_str(
            "position fen rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1 moves e2e4",
        )
        .unwrap();

        assert_eq!(
            command,
            UCIEngineCommand::Position(
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1".to_string(),
                vec!["e2e4".to_string()]
            )
        );
    }

    #[test]
    fn parse_engine_go() {
        let search_params = UCISearchParams {
            search_moves: vec!["e2e4".to_string()],
            ponder: true,
            w_time: Some(1),
            b_time: Some(2),
            w_inc: Some(3),
            b_inc: Some(4),
            moves_to_go: Some(5),
            depth: Some(6),
            nodes: Some(7),
            mate: Some(8),
            movetime: Some(9),
            infinite: true,
        };

        let command = UCIEngineCommand::from_str("go searchmoves e2e4 ponder wtime 1 btime 2 winc 3 binc 4 movestogo 5 depth 6 nodes 7 mate 8 movetime 9 infinite").unwrap();

        assert_eq!(command, UCIEngineCommand::Go(search_params))
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
