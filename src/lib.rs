pub(crate) mod app;
pub(crate) mod command;
pub(crate) mod context;
pub(crate) mod error;
pub(crate) mod flag;
pub(crate) mod handler;
pub(crate) mod parser;
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

    pub use crate::app::App;
    pub use crate::command::CommandDef;
    pub use crate::context::Context;
    pub use crate::error::{KoralError, KoralResult};
    pub use crate::flag::{Flag, FlagDef};
    pub use crate::traits::{App as AppTrait, FlagValue, FromArgs};
    pub use koral_derive::{App, Flag, Subcommand};
}

pub use app::App;
pub use command::CommandDef;
pub use context::Context;
pub use error::{KoralError, KoralResult};
pub use flag::{Flag, FlagDef};
pub use koral_derive::{App, Flag, Subcommand};
pub use traits::{FlagValue, FromArgs};
