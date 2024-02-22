use std::fmt::Display;

use crate::uci::UCIEngineCommandParseError;

#[derive(Debug, PartialEq)]
pub struct UCIOptionParams {
    pub id: String,
    pub value: Option<String>,
}

impl UCIOptionParams {
    pub fn from_tokens(tokens: Vec<&str>) -> Result<UCIOptionParams, UCIEngineCommandParseError> {
        enum Token {
            Empty,
            Name,
            Value,
        }

        let mut active_token = Token::Empty;
        let mut option_params = UCIOptionParams {
            id: "".to_string(),
            value: None,
        };

        let mut ids = Vec::new();
        let mut values = Vec::new();

        for token in tokens {
            match token {
                "setoption" => continue,
                "name" => active_token = Token::Name,
                "value" => active_token = Token::Value,
                _ => match active_token {
                    Token::Name => ids.push(token),
                    Token::Value => values.push(token),
                    Token::Empty => (),
                },
            }
        }

        option_params.id = ids.join(" ");
        if !values.is_empty() {
            option_params.value = Some(values.join(" "));
        }

        Ok(option_params)
    }
}

impl Display for UCIOptionParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UCIOptionParams { id, value: None } => write!(f, "name {}", id),
            UCIOptionParams {
                id,
                value: Some(value),
            } => write!(f, "name {} value {}", id, value),
        }
    }
}
