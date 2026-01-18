#[derive(Debug, Clone)]
/// Definition of a subcommand
pub struct CommandDef {
    /// Name of the command
    pub name: String,
    /// Description of the command
    pub description: String,
}

impl CommandDef {
    /// Create a new CommandDef
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
        }
    }
}
