#![warn(missing_docs)]
#![doc(html_logo_url = "https://github.com/ksk001100/koral/raw/main/assets/logo.png")]

//! # Koral
//!
//! > **Note**: This is an experimental project.
//!
//! A macro-centric CLI framework for Rust emphasizing separation of definition and state.
//!
//! Koral provides a declarative way to build Command Line Interfaces. By leveraging Rust's type system and procedural macros, it allows you to define your application's structure (flags, subcommands, metadata) separately from its runtime state.
//!
//! ## Philosophy: Definition vs. State
//!
//! Unlike other CLI libraries where the parsed result *is* your struct, Koral separates them:
//!
//! 1.  **Definition**: You define **Flags** and **Apps** as structs/enums with attributes. These represent *what* your CLI accepts.
//! 2.  **State**: At runtime, Koral parses arguments into a `Context`. Your application logic receives this context (or extracts values from it) and processes data.
//!
//! ## Key Features
//!
//! - **Declarative Macros**: Use `#[derive(App)]`, `#[derive(Subcommand)]`, and `#[derive(Flag)]`.
//! - **Dependency Injection**: Defines handlers that extract States and Flags directly (`fn run(state: State<S>, verbose: FlagVal<V>)`).
//! - **Middleware**: Hook into lifecycle execution (`before`/`after`) for logging, auth, etc. Supports both static registration and dynamic injection.
//! - **Type-Safe**: Flags are strongly typed. Custom types (Enums/Structs) supported via `#[derive(FlagValue)]`.
//! - **Extensible**: Share state easily across subcommands.
//! - **Validation**: strict mode, required flags, and custom validators.
//! - **Shell Completion**: Generate scripts for Bash, Zsh, and Fish.
//!
//! ## Installation
//!
//! ```bash
//! cargo add --git https://github.com/ksk001100/koral
//! ```
//!
//! ## Quick Start (Dependency Injection Style)
//!
//! ```rust
//! use koral::prelude::*;
//!
//! // 1. Define State
//! #[derive(Default, Clone)]
//! struct AppState {
//!     count: u32,
//! }
//!
//! // 2. Define Flags
//! #[derive(Flag, Debug)]
//! #[flag(name = "verbose", short = 'v', help = "Enable verbose output")]
//! struct VerboseFlag(bool);
//!
//! #[derive(Flag, Debug)]
//! #[flag(name = "name", default = "World", help = "Target name")]
//! struct NameFlag(String);
//!
//! // 3. Define App
//! #[derive(App)]
//! #[app(name = "greet", version = "1.0", action = run)]
//! #[app(flags(VerboseFlag, NameFlag))]
//! struct GreetApp;
//!
//! // 4. Define Handler with DI
//! // Koral automatically injects State, Flags, and Args!
//! fn run(
//!     state: State<AppState>,
//!     verbose: FlagArg<VerboseFlag>,
//!     name: FlagArg<NameFlag>
//! ) -> KoralResult<()> {
//!     if *verbose {
//!         println!("Debug: State count is {}", state.count);
//!     }
//!     println!("Hello, {}!", *name);
//!     Ok(())
//! }
//!
//! fn main() -> KoralResult<()> {
//!     // Run with state
//!     let mut state = AppState { count: 42 };
//!     let mut app = GreetApp;
//!     // In a real app, you might use: app.run_with_state(&mut state, std::env::args())
//!     // For testability here:
//!     let args = vec!["greet".to_string(), "--name".to_string(), "Koral".to_string()];
//!     app.run_with_state(&mut state, args)
//! }
//! ```
//!
//! ## Advanced Features
//!
//! ### Middleware (Hooks)
//!
//! You can define logic to run before and after your command.
//!
//! **Static Registration** (Simple):
//! ```rust
//! # use koral::prelude::*;
//! #[derive(Default)]
//! struct LoggerMiddleware;
//! impl Middleware for LoggerMiddleware {
//!     fn before(&self, _: &mut Context) -> KoralResult<()> {
//!         println!("Starting...");
//!         Ok(())
//!     }
//! }
//!
//! #[derive(App)]
//! #[app(name = "myapp")]
//! #[app(middleware(LoggerMiddleware))]
//! struct MyApp;
//! ```
//!
//! **Dynamic Injection** (Configurable):
//! ```rust
//! # use koral::prelude::*;
//! #[derive(Clone)]
//! struct AuthMiddleware { api_key: String }
//! impl Middleware for AuthMiddleware {
//!    fn before(&self, _: &mut Context) -> KoralResult<()> { Ok(()) }
//! }
//!
//! #[derive(App)]
//! #[app(name = "myapp")]
//! struct MyApp {
//!     #[app(middleware)] // Injects this field as middleware
//!     auth: AuthMiddleware
//! }
//!
//! fn main() {
//!     let app = MyApp {
//!         auth: AuthMiddleware { api_key: "secret".into() }
//!     };
//!     // app.run(args);
//! }
//! ```
//!
//! ### Flag Configuration
//!
//! Koral supports various attributes to customize flag behavior:
//!
//! - **required**: Marks the flag as mandatory (`required = true`).
//! - **env**: Sets an environment variable to read from if the flag is missing (`env = "MY_ENV_VAR"`).
//! - **value_name**: Customizes the placeholder name in help/completion (e.g. `value_name = "FILE"` -> `--config <FILE>`).
//! - **help_heading**: Groups the flag under a custom heading in the help message.
//! - **Strict Mode**: Add `#[app(strict)]` to treat unknown flags as errors instead of positional args.
//!
//! ```rust
//! # use koral::prelude::*;
//! #[derive(Flag)]
//! #[flag(
//!     name = "token",
//!     required = true,
//!     env = "API_TOKEN",
//!     help_heading = "Authentication"
//! )]
//! struct TokenFlag(String);
//!
//! #[derive(Flag)]
//! #[flag(
//!     name = "output",
//!     short = 'o',
//!     value_name = "PATH",
//!     help = "Output file path"
//! )]
//! struct OutputFlag(String);
//!
//! #[derive(App)]
//! #[app(name = "secure-app", strict)]
//! #[app(flags(TokenFlag, OutputFlag))]
//! struct SecureApp;
//! ```
//!
//! ### Custom Flag Types
//!
//! Easily parse Enums or Structs.
//!
//! ```rust
//! # use koral::prelude::*;
//! #[derive(FlagValue, Clone, Debug, PartialEq)] // Auto-implements FromStr/ToString
//! enum Format {
//!     Json,
//!     Text,
//! }
//!
//! #[derive(Flag)]
//! #[flag(name = "format", default = "text")]
//! struct FormatFlag(Format);
//! ```
//!
//! ### Shell Completion
//!
//! Generate completion scripts.
//!
//! ```rust,no_run
//! use koral::completion::{Shell, generate_to};
//! use koral::prelude::*;
//!
//! #[derive(App)]
//! #[app(name = "myapp")]
//! struct MyApp;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let app = MyApp;
//!     generate_to(&app, Shell::Bash, &mut std::io::stdout())?;
//!     Ok(())
//! }
//! ```

pub(crate) mod app;
pub(crate) mod command;
/// Shell completion generation.
pub mod completion;
pub(crate) mod context;
pub(crate) mod error;
/// Extractors for dependency injection.
pub mod extract;
pub(crate) mod flag;
pub(crate) mod handler;
/// Help message generation.
pub mod help;
/// Middlewares.
pub mod middleware;
/// Command line argument parser.
pub(crate) mod parser;
/// Core traits for the Koral framework.
pub mod traits;

#[doc(hidden)]
pub mod internal {
    pub mod app {
        pub use crate::app::*;
    }
    pub mod command {
        pub use crate::command::*;
    }
    pub mod context {
        pub use crate::context::*;
    }
    pub mod extract {
        pub use crate::extract::*;
    }
    pub mod error {
        pub use crate::error::*;
    }
    pub mod flag {
        pub use crate::flag::*;
    }
    pub mod handler {
        pub use crate::handler::*;
    }
    pub mod parser {
        pub use crate::parser::*;
    }
    pub mod traits {
        pub use crate::traits::*;
    }
}

pub mod prelude {
    //! The Koral prelude.
    //!
    //! The prelude allows users to import commonly used traits and types with a single line:
    //! ```rust
    //! use koral::prelude::*;
    //! ```
    //!
    //! This exports:
    //! - `App` (Struct)
    //! - `AppTrait` (Trait, aliased from `crate::traits::App`)
    //! - `Flag` (Trait and Derive Macro)
    //! - `FlagDef` (Struct)
    //! - `Context` (Struct)
    //! - `KoralResult` (Type Alias)
    //! - `KoralError` (Enum)
    //! - `App` (Derive Macro)
    //! - `Subcommand` (Derive Macro)
    //! - `FromArgs` (Trait)
    //! - `FlagValue` (Trait)
    //! - `CommandDef` (Struct)
    //! - `Middleware` (Trait)
    //! - `FromContext` (Trait)
    //! - `State` (Extractor)
    //! - `FlagArg` (Extractor)
    //! - `Args` (Extractor)

    pub use crate::app::App;
    pub use crate::command::CommandDef;
    pub use crate::context::Context;
    pub use crate::error::{KoralError, KoralResult};
    pub use crate::extract::{Args, FlagVal as FlagArg, FromContext, State};
    pub use crate::flag::{Flag, FlagDef};
    pub use crate::middleware::Middleware;
    pub use crate::traits::{App as AppTrait, FlagValue, FromArgs};
    pub use koral_derive::{App, Flag, FlagValue, Subcommand};
}

pub use app::App;
pub use command::CommandDef;
pub use completion::{generate_to, Shell};
pub use context::Context;
pub use error::{KoralError, KoralResult};
pub use extract::{Args, FlagVal as FlagArg, FromContext, State};
pub use flag::{Flag, FlagDef};
pub use koral_derive::{App, Flag, FlagValue, Subcommand};
pub use middleware::Middleware;
pub use traits::{FlagValue, FromArgs};
