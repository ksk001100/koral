use crate::context::Context;
use crate::error::KoralResult;
use crate::flag::{Flag, FlagDef};
use crate::traits::App as AppTrait;

/// The builder struct for defining an application.
pub struct App {
    name: String,
    version: String,
    description: String,
    flags: Vec<FlagDef>,
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

    /// Register a type-based flag.
    pub fn register<F: Flag + 'static>(mut self) -> Self {
        let def = FlagDef {
            name: F::name().to_string(),
            short: F::short(),
            long: F::long().map(|s| s.to_string()),
            help: F::help().to_string(),
            takes_value: F::takes_value(),
            default_value: F::default_value().map(|v| v.to_string()),
        };
        self.flags.push(def);
        self
    }

    /// Register a subcommand.
    pub fn subcommand<A: AppTrait + 'static>(mut self, sub: A) -> Self {
        self.subcommands.push(Box::new(sub));
        self
    }

    /// Set the action to be executed when the application runs.
    pub fn action<F>(mut self, action: F) -> Self
    where
        F: Fn(Context) -> KoralResult<()> + 'static,
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

    fn flags(&self) -> Vec<FlagDef> {
        self.flags.clone()
    }

    fn subcommands(&self) -> Vec<crate::command::CommandDef> {
        // This manual implementation of App struct is becoming tricky because it holds Box<dyn AppTrait>.
        // It needs to convert those into CommandDefs.
        self.subcommands
            .iter()
            .map(|b| crate::command::CommandDef::new(b.name(), b.description()))
            .collect()
    }

    fn execute(&mut self, ctx: Context) -> KoralResult<()> {
        if let Some(action) = &self.action {
            action(ctx)
        } else {
            Ok(())
        }
    }
}

// Manual Debug impl to skip action closure
impl std::fmt::Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App")
            .field("name", &self.name)
            .field("version", &self.version)
            .field("description", &self.description)
            .field("flags", &self.flags)
            // Skip subcommands if AppTrait usually doesn't strictly require Debug,
            // but subcommands usually interesting.
            // We can't debug subcommands if AppTrait doesn't require Debug.
            // Let's skip subcommands too or format len.
            .field("subcommands_count", &self.subcommands.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct VerboseFlag;
    impl Flag for VerboseFlag {
        type Value = bool;
        fn name() -> &'static str {
            "verbose"
        }
        fn short() -> Option<char> {
            Some('v')
        }
        fn takes_value() -> bool {
            false
        }
    }

    #[test]
    fn test_app_builder() {
        let app = App::new("test-app")
            .version("1.2.3")
            .description("A test app");

        assert_eq!(app.name(), "test-app");
        assert_eq!(AppTrait::version(&app), "1.2.3");
        assert_eq!(AppTrait::description(&app), "A test app");
    }

    #[test]
    fn test_app_execution() {
        use std::sync::{Arc, Mutex};

        let executed = Arc::new(Mutex::new(false));
        let executed_clone = executed.clone();

        let mut app = App::new("test")
            .register::<VerboseFlag>()
            .action(move |ctx| {
                let mut guard = executed_clone.lock().unwrap();
                *guard = true;
                // Verify context was passed
                assert!(ctx.get::<VerboseFlag>().unwrap_or(false));
                Ok(())
            });

        app.run(vec!["--verbose".to_string()]).unwrap();
        // Since bool defaults to takes_value=false but we didn't override it in VerboseFlag yet?
        // Wait, FlagValue for bool defaults takes_value = false.
        // So "--verbose" should be enough.
        // BUT my test registers VerboseFlag.
        // FlagValue trait impl for bool: if type_name=="bool" -> false.
        // So yes.
        // BUT the test passed arguments `vec!["--verbose".to_string(), "true".to_string()]`?
        // If takes_value is false, "true" would be seen as positional.
        // I should fix the test case args.
    }

    #[test]
    fn test_app_execution_bool() {
        use std::sync::{Arc, Mutex};
        let executed = Arc::new(Mutex::new(false));
        let executed_clone = executed.clone();

        let mut app = App::new("test")
            .register::<VerboseFlag>()
            .action(move |ctx| {
                *executed_clone.lock().unwrap() = true;
                assert!(ctx.get::<VerboseFlag>().unwrap_or(false));
                Ok(())
            });

        app.run(vec!["--verbose".to_string()]).unwrap();
        assert!(*executed.lock().unwrap());
    }
}
