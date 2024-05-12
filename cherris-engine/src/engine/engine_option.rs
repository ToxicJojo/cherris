use cherris_core::uci::{UCIOption, UCIOptionType};

#[derive(Clone)]
pub struct EngineOptions(pub [UCIOption; 1]);

impl EngineOptions {
    pub fn new() -> EngineOptions {
        let hash_option = UCIOption {
            id: "Hash".to_string(),
            option_type: UCIOptionType::Spin,
            default: Some("64".to_string()),
            min: Some("32".to_string()),
            max: Some("256".to_string()),
            var: vec![],
        };

        EngineOptions([hash_option])
    }
}
