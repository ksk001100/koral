#[derive(Debug, Clone)]
/// Definition of a subcommand
pub struct CommandDef {
    /// Name of the command
    pub name: String,
    /// Description of the command
    pub description: String,
    /// Aliases of the command
    pub aliases: Vec<String>,
    /// Nested subcommands
    pub subcommands: Vec<CommandDef>,
    /// Flags for this command
    pub flags: Vec<crate::flag::FlagDef>,
}

impl CommandDef {
    /// Create a new CommandDef
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            aliases: vec![],
            subcommands: vec![],
            flags: vec![],
        }
    }

    /// Set aliases
    pub fn with_aliases(mut self, aliases: Vec<String>) -> Self {
        self.aliases = aliases;
        self
    }

    /// Set subcommands
    pub fn with_subcommands(mut self, subcommands: Vec<CommandDef>) -> Self {
        self.subcommands = subcommands;
        self
    }

    /// Set flags
    pub fn with_flags(mut self, flags: Vec<crate::flag::FlagDef>) -> Self {
        self.flags = flags;
        self
    }
}
