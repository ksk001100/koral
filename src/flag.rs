use crate::traits::FlagValue;

/// Internal representation of a flag used by Parser and App.
#[derive(Clone, Debug)]
pub struct FlagDef {
    /// Name of the flag
    pub name: String,
    /// Short flag character (e.g. 'v')
    pub short: Option<char>,
    /// Long flag name
    pub long: Option<String>,
    /// Help text
    pub help: String,
    /// Whether the flag accepts a value
    pub takes_value: bool,
    /// Default value for the flag
    pub default_value: Option<String>,
    /// Environment variable name to read from
    pub env: Option<String>,
}

impl FlagDef {
    /// Create a FlagDef from a Flag trait implementation
    pub fn from_trait<F: Flag>() -> Self {
        Self {
            name: F::name().to_string(),
            short: F::short(),
            long: F::long().map(|s| s.to_string()),
            help: F::help().to_string(),
            takes_value: F::takes_value(),
            default_value: F::default_value().map(|v| v.to_string()),
            env: F::env().map(|s| s.to_string()),
        }
    }
}

// Re-defining for clarity and applying suggestion
/// Trait defining a command-line flag.
pub trait Flag
where
    <Self::Value as std::str::FromStr>::Err: std::fmt::Display,
{
    /// The type of value this flag holds.
    type Value: FlagValue;

    /// The canonical name of the flag.
    fn name() -> &'static str;

    /// Optional short character (e.g. 'v' for -v).
    fn short() -> Option<char> {
        None
    }

    /// Optional long name if different from name.
    fn long() -> Option<&'static str> {
        None
    }

    /// Help text.
    fn help() -> &'static str {
        ""
    }

    /// Whether the flag takes a value. Defaults to true.
    fn takes_value() -> bool {
        true
    }

    /// Default value if not provided.
    fn default_value() -> Option<Self::Value> {
        None
    }

    /// Environment variable to populate the flag from.
    fn env() -> Option<&'static str> {
        None
    }
}
