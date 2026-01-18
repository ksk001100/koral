use crate::traits::FlagValue;
use std::any::Any;
use std::collections::HashMap;

/// Result of parsing command line arguments.
#[derive(Debug, Default)]
pub struct Context<'a> {
    /// Parsed flag values. Key is the flag name.
    /// Value is the string representation of the value, or empty string for boolean flags.
    pub flags: HashMap<String, Option<String>>,

    /// Positional arguments.
    pub args: Vec<String>,

    /// Reference to the current command instance (e.g. AddCmd).
    pub app: Option<&'a mut dyn Any>,

    /// Reference to the shared state (e.g. TodoApp or TodoState).
    pub state: Option<&'a mut dyn Any>,
}

use crate::flag::Flag;

impl<'a> Context<'a> {
    pub fn new(flags: HashMap<String, Option<String>>, args: Vec<String>) -> Self {
        Self {
            flags,
            args,
            app: None,
            state: None,
        }
    }

    pub fn with_app(mut self, app: &'a mut dyn Any) -> Self {
        self.app = Some(app);
        self
    }

    pub fn with_state(mut self, state: &'a mut dyn Any) -> Self {
        self.state = Some(state);
        self
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

    /// Access the command instance as a specific type.
    pub fn app<T: Any>(&self) -> Option<&T> {
        // self.app is Option<&mut dyn Any>
        // We need to reborrow it as &Any to downcast_ref
        // self.app.as_ref().map(|a| a.downcast_ref::<T>()).flatten() // nested options
        if let Some(a) = &self.app {
            a.downcast_ref::<T>()
        } else {
            None
        }
    }

    /// Access the command instance mutably.
    pub fn app_mut<T: Any>(&mut self) -> Option<&mut T> {
        if let Some(a) = &mut self.app {
            a.downcast_mut::<T>()
        } else {
            None
        }
    }

    /// Access the shared state as a specific type.
    pub fn state<T: Any>(&self) -> Option<&T> {
        if let Some(s) = &self.state {
            s.downcast_ref::<T>()
        } else {
            None
        }
    }

    /// Access the shared state mutably.
    pub fn state_mut<T: Any>(&mut self) -> Option<&mut T> {
        if let Some(s) = &mut self.state {
            s.downcast_mut::<T>()
        } else {
            None
        }
    }
}
