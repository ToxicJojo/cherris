use std::fmt::Display;

pub struct UCIOption {
    pub id: String,
    pub option_type: UCIOptionType,
    pub default: Option<String>,
    pub min: Option<String>,
    pub max: Option<String>,
    pub var: Vec<String>,
}

pub enum UCIOptionType {
    Check,
    Spin,
    Combo,
    Button,
    String,
}

impl Display for UCIOptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UCIOptionType::Check => write!(f, "check"),
            UCIOptionType::Spin => write!(f, "spin"),
            UCIOptionType::Combo => write!(f, "combo"),
            UCIOptionType::Button => write!(f, "button"),
            UCIOptionType::String => write!(f, "string"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(UCIOptionType::Check.to_string(), "check");
        assert_eq!(UCIOptionType::Spin.to_string(), "spin");
        assert_eq!(UCIOptionType::Combo.to_string(), "combo");
        assert_eq!(UCIOptionType::Button.to_string(), "button");
        assert_eq!(UCIOptionType::String.to_string(), "string");
    }
}
