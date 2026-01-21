use crate::flag::FlagDef;

/// Trait for providing values for flags from various sources.
pub trait ValueProvider {
    /// Try to get a value for the given flag.
    fn get_value(&self, flag: &FlagDef) -> Option<String>;
}

/// Provides values from environment variables.
pub struct EnvProvider;

impl ValueProvider for EnvProvider {
    fn get_value(&self, flag: &FlagDef) -> Option<String> {
        if let Some(env_var) = &flag.env {
            if let Ok(val) = std::env::var(env_var) {
                return Some(val);
            }
        }
        None
    }
}

/// Provides values from default values defined in flags.
pub struct DefaultProvider;

impl ValueProvider for DefaultProvider {
    fn get_value(&self, flag: &FlagDef) -> Option<String> {
        flag.default_value.clone()
    }
}
