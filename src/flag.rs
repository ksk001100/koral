use crate::traits::FlagValue;

/// A command line flag.
///
/// Flags are used to parse named arguments (e.g. `--verbose`, `--count 3`).
/// They are generic over the type `T` which implements `FlagValue`.
#[derive(Clone, Debug)]
pub struct Flag<T: FlagValue> {
    pub name: String,
    pub aliases: Vec<String>,
    pub description: String,
    pub default_value: Option<T>,
}

impl<T: FlagValue> Flag<T> {
    /// Create a new flag with the given name.
    ///
    /// The name should be specified without the leading dashes (e.g. "verbose" for `--verbose`).
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            aliases: Vec::new(),
            description: String::new(),
            default_value: None,
        }
    }

    /// Add an alias for the flag.
    ///
    /// Aliases are usually short versions (e.g. "v" for `-v`).
    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.aliases.push(alias.into());
        self
    }

    /// Set the description of the flag for help messages.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Set a default value for the flag if it is not present in the arguments.
    pub fn default_value(mut self, value: T) -> Self {
        self.default_value = Some(value);
        self
    }

}

impl<T: FlagValue> crate::traits::Flag for Flag<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn aliases(&self) -> Vec<&str> {
        self.aliases.iter().map(|s| s.as_str()).collect()
    }

    fn takes_value(&self) -> bool {
        T::takes_value()
    }
}

