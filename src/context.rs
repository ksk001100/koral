use crate::traits::FlagValue;
use std::collections::HashMap;

/// Result of parsing command line arguments.
#[derive(Debug, Default)]
pub struct Context {
    /// Parsed flag values. Key is the flag name.
    /// Value is the string representation of the value, or empty string for boolean flags.
    pub flags: HashMap<String, Option<String>>,
    
    /// Positional arguments.
    pub args: Vec<String>,
}

impl Context {
    pub fn new(flags: HashMap<String, Option<String>>, args: Vec<String>) -> Self {
        Self { flags, args }
    }

    /// Check if a flag was present.
    pub fn has_flag(&self, name: &str) -> bool {
        self.flags.contains_key(name)
    }

    /// Get a flag value.
    /// Returns:
    /// - `Some(val)` if the flag was present and parsed successfully.
    /// - `None` if the flag was not present.
    /// - Panics or returns error if parsing fails? For now let's use Option.
    /// But wait, FlagValue::from_str returns Result.
    /// 
    /// If the flag is boolean, presence implies true.
    pub fn get<T: FlagValue>(&self, name: &str) -> Option<T> {
        if let Some(val_opt) = self.flags.get(name) {
            match val_opt {
                Some(s) => T::from_str(s).ok(),
                None => {
                    // This case happens for flags that don't take values (like bools).
                    // If T is bool, we should return true.
                    // If T is not bool, and we have no value, that's an issue unless we treated it as empty string.
                    // But our parser logic will decide this.
                    // For now, let's try to parse "true" if T is bool.
                    if !T::takes_value() {
                         T::from_str("true").ok()
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        }
    }
}
