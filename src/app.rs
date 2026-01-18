use crate::error::KoralResult;
use crate::traits::{App as AppTrait, Flag as FlagTrait};
use crate::flag::Flag;
use crate::traits::FlagValue;
use crate::context::Context;

/// The default implementation of a CLI application.
///
/// This struct allows for constructing a CLI using the builder pattern.
/// It implements `koral::traits::App` so it benefits from the standard lifecycle.
pub struct App {
    name: String,
    version: String,
    description: String,
    flags: Vec<Box<dyn FlagTrait>>,
    subcommands: Vec<Box<dyn AppTrait>>,
    action: Option<Box<dyn Fn(Context) -> KoralResult<()>>>,
}

impl App {
    /// Create a new application with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: "0.0.0".to_string(),
            description: String::new(),
            flags: Vec::new(),
            subcommands: Vec::new(),
            action: None,
        }
    }

    /// Set the version of the application.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    /// Set the description of the application.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Register a flag.
    pub fn flag<T: FlagValue>(mut self, flag: Flag<T>) -> Self {
        self.flags.push(Box::new(flag));
        self
    }

    /// Register a subcommand.
    pub fn subcommand<A: AppTrait + 'static>(mut self, sub: A) -> Self {
        self.subcommands.push(Box::new(sub));
        self
    }

    /// Set the action to be executed when the application runs.
    ///
    /// The action receives the `Context` containing parsed flags and arguments.
    pub fn action<F>(mut self, action: F) -> Self 
    where F: Fn(Context) -> KoralResult<()> + 'static 
    {
        self.action = Some(Box::new(action));
        self
    }

    /// Run the application with the given arguments.
    pub fn run(&mut self, args: Vec<String>) -> KoralResult<()> {
        crate::traits::App::run(self, args)
    }
}

impl AppTrait for App {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn flags(&self) -> Vec<&dyn FlagTrait> {
        self.flags.iter().map(|b| b.as_ref()).collect()
    }

    fn subcommands(&self) -> Vec<&dyn AppTrait> {
        self.subcommands.iter().map(|b| b.as_ref()).collect()
    }

    fn execute(&mut self, ctx: Context) -> KoralResult<()> {
        if let Some(action) = &self.action {
            action(ctx)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_builder() {
        let app = App::new("test-app")
            .version("1.2.3")
            .description("A test app");
        
        assert_eq!(app.name(), "test-app");
        assert_eq!(app.version(), "1.2.3");
        assert_eq!(app.description(), "A test app");
    }

    #[test]
    fn test_app_execution() {
        use std::sync::{Arc, Mutex};
        
        let executed = Arc::new(Mutex::new(false));
        let executed_clone = executed.clone();

        let mut app = App::new("test")
            .flag(Flag::<bool>::new("verbose"))
            .action(move |ctx| {
                let mut guard = executed_clone.lock().unwrap();
                *guard = true;
                // Verify context was passed
                assert!(ctx.has_flag("verbose"));
                Ok(())
            });

        app.run(vec!["--verbose".to_string()]).unwrap();
        
        assert!(*executed.lock().unwrap());
    }
}

