use crate::traits::App;
use clap_complete::{generate, Shell as ClapShell};
use std::io::{self, Write};

/// Supported shells for completion generation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shell {
    /// Bourne Again SHell
    Bash,
    /// Z Shell
    Zsh,
    /// Friendly Interactive SHell
    Fish,
}

impl From<Shell> for ClapShell {
    fn from(s: Shell) -> Self {
        match s {
            Shell::Bash => ClapShell::Bash,
            Shell::Zsh => ClapShell::Zsh,
            Shell::Fish => ClapShell::Fish,
        }
    }
}

/// Generate completion script for the given app and shell
pub fn generate_to<W: Write>(app: &impl App, shell: Shell, buf: &mut W) -> io::Result<()> {
    let mut cmd = crate::parser::build_command(app);
    let name = app.name().to_string();
    let clap_shell: ClapShell = shell.into();
    generate(clap_shell, &mut cmd, name, buf);
    Ok(())
}
