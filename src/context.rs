use crate::traits::FlagValue;
use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Result of parsing command line arguments.
/// Result of parsing command line arguments.
/// Reference to the shared state (e.g. TodoApp or TodoState).
pub struct Context<'a, A: ?Sized = dyn Any> {
    /// Parsed flag values. Key is the flag name.
    /// Value is the string representation of the value, or empty string for boolean flags.
    pub flags: HashMap<String, Option<String>>,

    /// Positional arguments.
    pub args: Vec<String>,

    /// Reference to the current command instance (e.g. AddCmd).
    pub app: Option<&'a mut A>,

    /// Reference to the shared state (e.g. TodoApp or TodoState).
    pub state: Option<&'a mut dyn Any>,

    /// Type-safe extensions map for middleware to inject data.
    pub extensions: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

use crate::flag::Flag;

impl<'a, A: ?Sized> Context<'a, A> {
    /// Create a new Context.
    pub fn new(flags: HashMap<String, Option<String>>, args: Vec<String>) -> Self {
        Self {
            flags,
            args,
            app: None,
            state: None,
            extensions: HashMap::new(),
        }
    }

    /// Set the application reference.
    pub fn with_app(mut self, app: &'a mut A) -> Self {
        self.app = Some(app);
        self
    }

    /// Set the shared state reference.
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
    /// Check if a flag is present (alias for is_present).
    pub fn has_flag(&self, name: &str) -> bool {
        self.is_present(name)
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

    /// Insert an extension into the context.
    pub fn insert_extension<T: Any + Send + Sync>(&mut self, val: T) {
        self.extensions.insert(TypeId::of::<T>(), Box::new(val));
    }

    /// Get a reference to an extension.
    pub fn get_extension<T: Any + Send + Sync>(&self) -> Option<&T> {
        self.extensions
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref())
    }

    /// Get a mutable reference to an extension.
    pub fn get_extension_mut<T: Any + Send + Sync>(&mut self) -> Option<&mut T> {
        self.extensions
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_mut())
    }

    /// Remove an extension from the context.
    pub fn remove_extension<T: Any + Send + Sync>(&mut self) -> Option<T> {
        self.extensions
            .remove(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast::<T>().ok())
            .map(|boxed| *boxed)
    }

    /// Set the extensions map (used internally).
    pub fn with_extensions(
        mut self,
        extensions: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    ) -> Self {
        self.extensions = extensions;
        self
    }
}

// Helpers for dynamic context (Context<'a, dyn Any>)
impl<'a> Context<'a, dyn Any> {
    /// Access the command instance as a specific type.
    pub fn app<T: Any>(&self) -> Option<&T> {
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
}

impl<'a, A: ?Sized> std::fmt::Debug for Context<'a, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("flags", &self.flags)
            .field("args", &self.args)
            .field("app", &"...")
            .field("state", &"...")
            .field("extensions", &self.extensions.keys())
            .finish()
    }
}

impl<'a, A: ?Sized> Default for Context<'a, A> {
    fn default() -> Self {
        Self {
            flags: HashMap::new(),
            args: Vec::new(),
            app: None,
            state: None,
            extensions: HashMap::new(),
        }
    }
}
