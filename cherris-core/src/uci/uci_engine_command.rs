use std::{fmt::Display, num::ParseIntError, str::FromStr};

use crate::{
    uci::UCIGoParams, uci::UCIOptionParams, uci::UCIPositionParams, uci::UCIRegisterParams,
};

/// These are all the command the engine gets from the interface.
#[derive(Debug, PartialEq)]
pub enum UCIEngineCommand {
    /// - Tell engine to use the uci (universal chess interface), this will be sent once as a first command after program boot to tell the engine to switch to uci mode.
    /// - After receiving the uci command the engine must identify itself with the id command and send the option commands to tell the GUI which engine settings the engine supports if any.
    /// - After that the engine should send uciok to acknowledge the uci mode. If no uciok is sent within a certain time period, the engine task will be killed by the GUI.
    Uci,
    /// - Switch the debug mode of the engine on and off. In debug mode the engine should send additional infos to the GUI, e.g. with the info string command, to help debugging, e.g. the commands that the engine has received etc.
    /// - This mode should be switched off by default and this command can be sent any time, also when the engine is thinking.
    Debug(bool),
    /// - This is used to synchronize the engine with the GUI. When the GUI has sent a command or multiple commands that can take some time to complete, this command can be used to wait for the engine to be ready again or to ping the engine to find out if it is still alive. E.g. this should be sent after setting the path to the tablebases as this can take some time.
    /// - This command is also required once before the engine is asked to do any search to wait for the engine to finish initializing.
    /// - This command must always be answered with readyok and can be sent also when the engine is calculating in which case the engine should also immediately answer with readyok without stopping the search.
    IsReady,
    /// - This is sent to the engine when the user wants to change the internal parameters of the engine. For the button type no value is needed.
    /// - One string will be sent for each parameter and this will only be sent when the engine is waiting. The name and value of the option in id should not be case sensitive and can inlude spaces.
    /// - The substrings value and name should be avoided in id and x to allow unambiguous parsing, for example do not use name = draw value.
    SetOption(UCIOptionParams),
    /// - This is the command to try to register an engine or to tell the engine that registration will be done later. This command should always be sent if the engine has sent registration error at program startup.
    Register(UCIRegisterParams),
    /// - This is sent to the engine when the next search (started with position and go) will be from a different game. This can be a new game the engine should play or a new game it should analyse but also the next position from a testsuite with positions only.
    /// - If the GUI hasn't sent a ucinewgame before the first position command, the engine shouldn't expect any further ucinewgame commands as the GUI is probably not supporting the ucinewgame command. So the engine should not rely on this command even though all new GUIs should support it.
    /// - As the engine's reaction to ucinewgame can take some time the GUI should always send isready after ucinewgame to wait for the engine to finish its operation.
    UciNewGame,
    /// - Set up the position described in fenstring on the internal board and play the moves on the internal chess board.
    /// - If the game was played from the start position the string startpos will be sent.
    /// - Note: no "new" command is needed. However, if this position is from a different game than the last position sent to the engine, the GUI should have sent a ucinewgame inbetween.
    Position(UCIPositionParams),
    /// - Start calculating on the current position set up with the position command.
    /// - There are a number of commands that can follow this command, all will be sent in the same string. If one command is not sent its value should be interpreted as it would not influence the search.
    Go(UCIGoParams),
    /// - Stop calculating as soon as possible,
    /// - Don't forget the bestmove and possibly the ponder token when finishing the search
    Stop,
    /// - The user has played the expected move. This will be sent if the engine was told to ponder on the same move the user has played. The engine should continue searching but switch from pondering to normal search.
    PonderHit,
    /// - Quit the program as soon as possible.
    Quit,
}

#[derive(Debug, PartialEq)]
pub enum UCIEngineCommandParseError {
    InvalidCommand,
    InvalidToken,
    ParseInt,
}

impl From<ParseIntError> for UCIEngineCommandParseError {
    fn from(_: ParseIntError) -> Self {
        UCIEngineCommandParseError::ParseInt
    }
}

pub type ParseResult = Result<UCIEngineCommand, UCIEngineCommandParseError>;

impl FromStr for UCIEngineCommand {
    type Err = UCIEngineCommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        let command = tokens[0];

        match command {
            "uci" => Ok(UCIEngineCommand::Uci),
            "debug" => match tokens[1] {
                "on" => Ok(UCIEngineCommand::Debug(true)),
                "off" => Ok(UCIEngineCommand::Debug(false)),
                _ => Err(UCIEngineCommandParseError::InvalidCommand),
            },
            "isready" => Ok(UCIEngineCommand::IsReady),
            "setoption" => {
                let params = UCIOptionParams::from_tokens(tokens)?;
                Ok(UCIEngineCommand::SetOption(params))
            }
            "register" => {
                let params = UCIRegisterParams::from_tokens(tokens)?;
                Ok(UCIEngineCommand::Register(params))
            }
            "ucinewgame" => Ok(UCIEngineCommand::UciNewGame),
            "position" => {
                let params = UCIPositionParams::from_token(tokens)?;
                Ok(UCIEngineCommand::Position(params))
            }
            "go" => {
                let params = UCIGoParams::from_tokens(tokens)?;
                Ok(UCIEngineCommand::Go(params))
            }
            "stop" => Ok(UCIEngineCommand::Stop),
            "ponderhit" => Ok(UCIEngineCommand::PonderHit),
            "quit" => Ok(UCIEngineCommand::Quit),
            _ => Err(UCIEngineCommandParseError::InvalidCommand),
        }
    }
}

impl Display for UCIEngineCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UCIEngineCommand::Uci => writeln!(f, "uci"),
            UCIEngineCommand::Debug(true) => writeln!(f, "debug on"),
            UCIEngineCommand::Debug(false) => writeln!(f, "debug off"),
            UCIEngineCommand::IsReady => writeln!(f, "isready"),
            UCIEngineCommand::SetOption(params) => writeln!(f, "setoption {}", params),
            UCIEngineCommand::Register(params) => writeln!(f, "register {}", params),
            UCIEngineCommand::UciNewGame => writeln!(f, "ucinewgame"),
            UCIEngineCommand::Position(params) => writeln!(f, "position {}", params),
            UCIEngineCommand::Go(params) => writeln!(f, "go{}", params),
            UCIEngineCommand::Stop => writeln!(f, "stop"),
            UCIEngineCommand::PonderHit => writeln!(f, "ponderhit"),
            UCIEngineCommand::Quit => writeln!(f, "quit"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Position;

    use super::*;

    #[test]
    fn parse_invalid_command() {
        let command = UCIEngineCommand::from_str("invalid");

        assert_eq!(command, Err(UCIEngineCommandParseError::InvalidCommand));
    }

    #[test]
    fn parse_uci() {
        let command = UCIEngineCommand::from_str("uci").unwrap();

        assert_eq!(command, UCIEngineCommand::Uci);
    }

    #[test]
    fn display_uci() {
        assert_eq!(UCIEngineCommand::Uci.to_string(), "uci\n");
    }

    #[test]
    fn parse_is_ready() {
        let command = UCIEngineCommand::from_str("isready").unwrap();

        assert_eq!(command, UCIEngineCommand::IsReady);
    }

    #[test]
    fn display_is_ready() {
        assert_eq!(UCIEngineCommand::IsReady.to_string(), "isready\n");
    }

    #[test]
    fn parse_debug() {
        let command_on = UCIEngineCommand::from_str("debug on").unwrap();
        let command_off = UCIEngineCommand::from_str("debug off").unwrap();

        assert_eq!(command_on, UCIEngineCommand::Debug(true));
        assert_eq!(command_off, UCIEngineCommand::Debug(false));
    }

    #[test]
    fn display_debug() {
        assert_eq!(UCIEngineCommand::Debug(true).to_string(), "debug on\n");
        assert_eq!(UCIEngineCommand::Debug(false).to_string(), "debug off\n");
    }

    #[test]
    fn parse_set_option() {
        let command = UCIEngineCommand::from_str("setoption name Nullmove value true").unwrap();
        let option_params = UCIOptionParams {
            id: "Nullmove".to_string(),
            value: Some("true".to_string()),
        };

        assert_eq!(command, UCIEngineCommand::SetOption(option_params));
    }

    #[test]
    fn parse_set_option_no_value() {
        let command = UCIEngineCommand::from_str("setoption name Clear Hash").unwrap();
        let option_params = UCIOptionParams {
            id: "Clear Hash".to_string(),
            value: None,
        };

        assert_eq!(command, UCIEngineCommand::SetOption(option_params));
    }

    #[test]
    fn display_set_option() {
        let option_params = UCIOptionParams {
            id: "Nullmove".to_string(),
            value: Some("true".to_string()),
        };

        assert_eq!(
            UCIEngineCommand::SetOption(option_params).to_string(),
            "setoption name Nullmove value true\n"
        );
    }

    #[test]
    fn display_set_option_no_value() {
        let option_params = UCIOptionParams {
            id: "Clear Hash".to_string(),
            value: None,
        };

        assert_eq!(
            UCIEngineCommand::SetOption(option_params).to_string(),
            "setoption name Clear Hash\n"
        );
    }

    #[test]
    fn parse_register_later() {
        let command = UCIEngineCommand::from_str("register later").unwrap();

        assert_eq!(
            command,
            UCIEngineCommand::Register(UCIRegisterParams::Later)
        )
    }

    #[test]
    fn parse_register() {
        let command = UCIEngineCommand::from_str("register name Stefan MK code 435987").unwrap();

        assert_eq!(
            command,
            UCIEngineCommand::Register(UCIRegisterParams::Register {
                name: "Stefan MK".to_string(),
                code: "435987".to_string()
            })
        )
    }

    #[test]
    fn displat_register_later() {
        assert_eq!(
            UCIEngineCommand::Register(UCIRegisterParams::Later).to_string(),
            "register later\n"
        );
    }

    #[test]
    fn display_register() {
        assert_eq!(
            UCIEngineCommand::Register(UCIRegisterParams::Register {
                name: "Stefan MK".to_string(),
                code: "435987".to_string()
            })
            .to_string(),
            "register name Stefan MK code 435987\n"
        );
    }

    #[test]
    fn parse_ucinewgame() {
        let command = UCIEngineCommand::from_str("ucinewgame").unwrap();

        assert_eq!(command, UCIEngineCommand::UciNewGame);
    }

    #[test]
    fn display_ucinewgame() {
        assert_eq!(UCIEngineCommand::UciNewGame.to_string(), "ucinewgame\n");
    }

    #[test]
    fn parse_position_startpos() {
        let command = UCIEngineCommand::from_str("position startpos moves e2e4").unwrap();

        assert_eq!(
            command,
            UCIEngineCommand::Position(UCIPositionParams {
                fen: Position::STARTING_FEN.to_string(),
                moves: vec!["e2e4".to_string()],
            })
        )
    }

    #[test]
    fn parse_position_fen() {
        let command = UCIEngineCommand::from_str("position fen rnbqk2r/ppp2ppp/3b1n2/3p4/3P4/2N1PN2/PP3PPP/R1BQKB1R b KQkq - 3 6  moves e2e4").unwrap();

        assert_eq!(
            command,
            UCIEngineCommand::Position(UCIPositionParams {
                fen: "rnbqk2r/ppp2ppp/3b1n2/3p4/3P4/2N1PN2/PP3PPP/R1BQKB1R b KQkq - 3 6"
                    .to_string(),
                moves: vec!["e2e4".to_string()],
            })
        )
    }

    #[test]
    fn display_position_startpos() {
        assert_eq!(
            UCIEngineCommand::Position(UCIPositionParams::startpos(vec![])).to_string(),
            "position startpos\n"
        )
    }

    #[test]
    fn display_position_fen() {
        assert_eq!(
            UCIEngineCommand::Position(UCIPositionParams {
                fen: "rnbqk2r/ppp2ppp/3b1n2/3p4/3P4/2N1PN2/PP3PPP/R1BQKB1R b KQkq - 3 6".to_owned(),
                moves: vec!["e2e4".to_string()]
            })
            .to_string(),
            "position fen rnbqk2r/ppp2ppp/3b1n2/3p4/3P4/2N1PN2/PP3PPP/R1BQKB1R b KQkq - 3 6 moves e2e4\n"
        )
    }

    #[test]
    fn parse_go() {
        let command = UCIEngineCommand::from_str("go searchmoves e2e4 ponder wtime 1 btime 2 winc 3 binc 4 movestogo 5 depth 6 nodes 7 mate 8 movetime 9 infinite").unwrap();

        assert_eq!(
            command,
            UCIEngineCommand::Go(UCIGoParams {
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
            })
        )
    }

    #[test]
    fn display_go() {
        let command = UCIEngineCommand::Go(UCIGoParams {
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
        });

        assert_eq!(
            command.to_string(),
            "go searchmoves e2e4 ponder wtime 1 btime 2 winc 3 binc 4 movestogo 5 depth 6 nodes 7 mate 8 movetime 9 infinite\n"
        );
    }

    #[test]
    fn parse_stop() {
        let command = UCIEngineCommand::from_str("stop").unwrap();

        assert_eq!(command, UCIEngineCommand::Stop)
    }

    #[test]
    fn display_stop() {
        assert_eq!(UCIEngineCommand::Stop.to_string(), "stop\n");
    }

    #[test]
    fn parse_ponderhit() {
        let command = UCIEngineCommand::from_str("ponderhit").unwrap();

        assert_eq!(command, UCIEngineCommand::PonderHit);
    }

    #[test]
    fn display_ponderhit() {
        assert_eq!(UCIEngineCommand::PonderHit.to_string(), "ponderhit\n");
    }

    #[test]
    fn parse_quit() {
        let command = UCIEngineCommand::from_str("quit").unwrap();

        assert_eq!(command, UCIEngineCommand::Quit);
    }

    #[test]
    fn display_quit() {
        assert_eq!(UCIEngineCommand::Quit.to_string(), "quit\n");
    }
}
