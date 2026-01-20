// use koral::prelude::*; // Unused
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct OutputConfig {
    pub format: OutputFormat,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Copy)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
    Yaml,
    Table,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(OutputFormat::Text),
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "table" => Ok(OutputFormat::Table),
            _ => Err(format!("Unknown format: {}", s)),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::Text => write!(f, "text"),
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Yaml => write!(f, "yaml"),
            OutputFormat::Table => write!(f, "table"),
        }
    }
}

pub fn print_output<T: Serialize + fmt::Debug>(item: &T, format: OutputFormat) {
    match format {
        OutputFormat::Text => println!("{:?}", item),
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(item).unwrap()),
        OutputFormat::Yaml => println!("(YAML output not implemented for mock)\n{:?}", item),
        OutputFormat::Table => println!("(Table output not implemented for mock)\n{:?}", item),
    }
}
