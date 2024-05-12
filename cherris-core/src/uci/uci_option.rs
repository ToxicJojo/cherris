use std::fmt::Display;

/// An option that can be set on a UCI server.
#[derive(Clone)]
pub struct UCIOption {
    pub id: String,
    pub option_type: UCIOptionType,
    pub default: Option<String>,
    pub min: Option<String>,
    pub max: Option<String>,
    pub var: Vec<String>,
}

/// The type of option. There are 5 different types of options the engine can send.
#[derive(Clone)]
pub enum UCIOptionType {
    /// A checkbox that can either be true or false
    Check,
    /// A spin wheel that can be an integer in a certain range
    Spin,
    /// A combo box that can have different predefined strings as a value
    Combo,
    /// A button that can be pressed to send a command to the engine
    Button,
    /// A text field that has a string as a value
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

impl Display for UCIOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name {} type {}", self.id, self.option_type)?;
        if let Some(default) = &self.default {
            write!(f, " default {}", default)?;
        }

        if let Some(min) = &self.min {
            write!(f, " min {}", min)?;
        }

        if let Some(max) = &self.max {
            write!(f, " max {}", max)?;
        }

        for var in &self.var {
            write!(f, " var {}", var)?;
        }

        Ok(())
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
