#[derive(Debug, Clone)]
/// Definition of a subcommand
pub struct CommandDef {
    /// Name of the command
    pub name: String,
    /// Description of the command
    pub description: String,
    /// Aliases of the command
    pub aliases: Vec<String>,
}

impl CommandDef {
    /// Create a new CommandDef
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            aliases: vec![],
        }
    }

    /// Set aliases
    pub fn with_aliases(mut self, aliases: Vec<String>) -> Self {
        self.aliases = aliases;
        self
    }
}
