use std::fmt::Display;

use crate::{uci::UCIOption, uci::UCISearchInfo};

/// Represents all UCI commands that can be sent to a GUI.
pub enum UCIGuiCommand {
    /// This must be sent after receiving the uci command to identify the engine.
    IdName(String),
    /// This must be sent after receiving the uci command to identify the engine.
    IdAuthor(String),
    /// Must be sent after the id and optional options to tell the GUI that the engine has sent all infos and is ready in uci mode.
    UciOk,
    /// - This must be sent when the engine has received an isready command and has processed all input and is ready to accept new commands now.
    /// - It is usually sent after a command that can take some time to be able to wait for the engine, but it can be used anytime, even when the engine is searching, and must always be answered with isready.
    ReadyOk,
    /// - The engine has stopped searching and found the move move best in this position.
    /// - The engine can send the move it likes to ponder on. The engine must not start pondering automatically.
    /// - This command must always be sent if the engine stops searching, also in pondering mode if there is a stop command, so for every go command a bestmove command is needed!
    /// - Directly before that the engine should send a final info command with the final search information, the the GUI has the complete statistics about the last search.
    BestMove(String),
    /// - This is needed for copyprotected engines. After the uciok command the engine can tell the GUI, that it will check the copy protection now. This is done by copyprotection checking.
    /// - If the check is ok the engine should send copyprotection ok, otherwise copyprotection error. If there is an error the engine should not function properly but should not quit alone. If the engine reports copyprotection error the GUI should not use this engine and display an error message instead!
    CopyProtectionChecking,
    CopyProtectionOk,
    CopyProtectionError,
    /// - This is needed for engines that need a username and/or a code to function with all features.
    /// - Analog to the copyprotection command the engine can send registration checking after the uciok command followed by either registration ok or registration error. Also after every attempt to register the engine it should answer with registration checking and then either registration ok or registration error.
    /// - In contrast to the copyprotection command, the GUI can use the engine after the engine has reported an error, but should inform the user that the engine is not properly registered and might not use all its features.
    /// - In addition the GUI should offer to open a dialog to enable registration of the engine. To try to register an engine the GUI can send the register command. The GUI has to always answer with the register command if the engine sends registration error at engine startup (this can also be done with register later) and tell the user somehow that the engine is not registered. This way the engine knows that the GUI can deal with the registration procedure and the user will be informed that the engine is not properly registered.
    RegistrationChecking,
    RegistrationOk,
    RegistrationError,
    /// - The engine wants to send information to the GUI. This should be done whenever one of the info has changed.
    Info(UCISearchInfo),
    /// - This command tells the GUI which parameters can be changed in the engine.
    /// - This should be sent once at engine startup after the uci and the id commands if any parameter can be changed in the engine.
    Option(UCIOption),
}

impl Display for UCIGuiCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UCIGuiCommand::IdName(name) => writeln!(f, "id name {}", name),
            UCIGuiCommand::IdAuthor(author) => writeln!(f, "id author {}", author),
            UCIGuiCommand::UciOk => writeln!(f, "uciok"),
            UCIGuiCommand::ReadyOk => writeln!(f, "readyok"),
            UCIGuiCommand::BestMove(mv) => writeln!(f, "bestmove {}", mv),
            UCIGuiCommand::CopyProtectionChecking => writeln!(f, "copyprotection checking"),
            UCIGuiCommand::CopyProtectionOk => writeln!(f, "copyprotection ok"),
            UCIGuiCommand::CopyProtectionError => writeln!(f, "copyprotection error"),
            UCIGuiCommand::RegistrationChecking => writeln!(f, "registration checking"),
            UCIGuiCommand::RegistrationOk => writeln!(f, "registration ok"),
            UCIGuiCommand::RegistrationError => writeln!(f, "registration error"),
            UCIGuiCommand::Info(info) => writeln!(f, "info {}", info),
            UCIGuiCommand::Option(option) => writeln!(f, "option {}", option),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::uci::UCIOptionType;

    use super::*;

    #[test]
    fn display_id_name() {
        assert_eq!(
            UCIGuiCommand::IdName("Cheris".to_string()).to_string(),
            "id name Cheris\n"
        );
    }

    #[test]
    fn display_id_author() {
        assert_eq!(
            UCIGuiCommand::IdAuthor("Johannes Thiel".to_string()).to_string(),
            "id author Johannes Thiel\n"
        );
    }

    #[test]
    fn display_uciok() {
        assert_eq!(UCIGuiCommand::UciOk.to_string(), "uciok\n");
    }

    #[test]
    fn display_readyok() {
        assert_eq!(UCIGuiCommand::ReadyOk.to_string(), "readyok\n");
    }

    #[test]
    fn display_bestmove() {
        assert_eq!(
            UCIGuiCommand::BestMove("e2e4".to_string()).to_string(),
            "bestmove e2e4\n"
        );
    }

    #[test]
    fn display_copyprotection_checking() {
        let command = UCIGuiCommand::CopyProtectionChecking.to_string();
        assert_eq!(command, "copyprotection checking\n")
    }

    #[test]
    fn display_copyprotection_ok() {
        let command = UCIGuiCommand::CopyProtectionOk.to_string();
        assert_eq!(command, "copyprotection ok\n")
    }

    #[test]
    fn display_copyprotection_error() {
        let command = UCIGuiCommand::CopyProtectionError.to_string();
        assert_eq!(command, "copyprotection error\n")
    }

    #[test]
    fn display_registration_checking() {
        let command = UCIGuiCommand::RegistrationChecking.to_string();
        assert_eq!(command, "registration checking\n")
    }

    #[test]
    fn display_registration_ok() {
        let command = UCIGuiCommand::RegistrationOk.to_string();
        assert_eq!(command, "registration ok\n")
    }

    #[test]
    fn display_registration_error() {
        let command = UCIGuiCommand::RegistrationError.to_string();
        assert_eq!(command, "registration error\n")
    }

    #[test]
    fn display_info() {
        let command = UCIGuiCommand::Info(UCISearchInfo {
            depth: 1,
            seldepth: 2,
            score: 3,
            time: 4,
            nodes: 5,
            pv: vec!["e2e4".to_string()],
            nps: 6,
        });
        assert_eq!(
            command.to_string(),
            "info depth 1 seldepth 2 score cp 3 time 4 nodes 5 nps 6 pv e2e4\n"
        );
    }

    #[test]
    fn display_option() {
        let command = UCIGuiCommand::Option(UCIOption {
            id: "Selectivity".to_string(),
            option_type: UCIOptionType::Spin,
            default: Some("2".to_string()),
            min: Some("0".to_string()),
            max: Some("4".to_string()),
            var: vec![],
        });

        assert_eq!(
            command.to_string(),
            "option name Selectivity type spin default 2 min 0 max 4\n"
        );
    }
}
