use crate::traits::FlagValue;
use std::collections::HashMap;

/// Result of parsing command line arguments.
#[derive(Debug, Default)]
pub struct Context<'a, A: ?Sized = ()> {
    /// Parsed flag values. Key is the flag name.
    /// Value is the string representation of the value, or empty string for boolean flags.
    pub flags: HashMap<String, Option<String>>,

    /// Positional arguments.
    pub args: Vec<String>,

    /// Reference to the application instance.
    pub app: Option<&'a mut A>,
}

use crate::flag::Flag;

impl<'a, A: ?Sized> Context<'a, A> {
    pub fn new(flags: HashMap<String, Option<String>>, args: Vec<String>) -> Self {
        Self {
            flags,
            args,
            app: None,
        }
    }

    pub fn with_app<'b, B: ?Sized>(self, app: &'b mut B) -> Context<'b, B> {
        Context {
            flags: self.flags,
            args: self.args,
            app: Some(app),
        }
    }

    /// Check if a flag was present.
    pub fn is_present(&self, name: &str) -> bool {
        self.flags.contains_key(name)
    }

    /// Get typed flag value using the Flag trait.
    pub fn get<F: Flag>(&self) -> Option<F::Value>
    where
        <F::Value as std::str::FromStr>::Err: std::fmt::Display,
    {
        self.value_t::<F::Value>(F::name()).ok()
    }

    /// Get raw flag value as string, if present.
    pub fn value_of(&self, name: &str) -> Option<&str> {
        self.flags.get(name).and_then(|opt| opt.as_deref())
    }

    /// Get typed flag value.
    pub fn value_t<T: FlagValue>(&self, name: &str) -> Result<T, String>
    where
        <T as std::str::FromStr>::Err: std::fmt::Display,
    {
        if let Some(opt_val) = self.flags.get(name) {
            match opt_val {
                Some(s) => T::from_str(s).map_err(|e| e.to_string()),
                None => {
                    // Special case for boolean flags which might be stored as None
                    if std::any::type_name::<T>() == "bool" {
                        T::from_str("true").map_err(|e| e.to_string())
                    } else {
                        Err(format!("Flag '{}' was used but provided no value", name))
                    }
                }
            }
        } else {
            Err(format!("Argument '{}' not found", name))
        }
    }

    // Deprecated or alias helper
    pub fn has_flag(&self, name: &str) -> bool {
        self.is_present(name)
    }
}
