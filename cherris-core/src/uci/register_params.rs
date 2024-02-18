use std::fmt::Display;

use crate::uci_engine_command::UCIEngineCommandParseError;

#[derive(Debug, PartialEq)]
pub enum UCIRegisterParams {
    Later,
    Register { name: String, code: String },
}

impl UCIRegisterParams {
    pub fn from_tokens(tokens: Vec<&str>) -> Result<UCIRegisterParams, UCIEngineCommandParseError> {
        enum Mode {
            Empty,
            Name,
            Code,
        }

        let mut mode = Mode::Empty;
        let mut name = Vec::new();
        let mut code = Vec::new();

        for token in tokens {
            match token {
                "register" => continue,
                "later" => return Ok(UCIRegisterParams::Later),
                "name" => mode = Mode::Name,
                "code" => mode = Mode::Code,
                _ => match mode {
                    Mode::Name => name.push(token),
                    Mode::Code => code.push(token),
                    Mode::Empty => (),
                },
            }
        }

        let register_params = UCIRegisterParams::Register {
            name: name.join(" "),
            code: code.join(" "),
        };
        Ok(register_params)
    }
}

impl Display for UCIRegisterParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UCIRegisterParams::Later => write!(f, "later"),
            UCIRegisterParams::Register { name, code } => write!(f, "name {} code {}", name, code),
        }
    }
}
