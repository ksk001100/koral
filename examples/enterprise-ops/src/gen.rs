use koral::completion::{generate_to, Shell};
use koral::prelude::*;
use std::io::{stdout, Write};

/// Generate shell completion scripts
#[derive(Default, App, Debug)]
#[app(
    name = "completion",
    help = "Generate shell completion scripts (bash, zsh, fish)",
    action = completion_action
)]
#[app(flags(ShellFlag))]
pub struct CompletionCmd;

#[derive(Flag, Debug, Clone)]
#[flag(
    name = "shell",
    short = 's',
    help = "Target shell (bash, zsh, fish)",
    required = true
)]
pub struct ShellFlag(String);

fn completion_action(_app: &mut CompletionCmd, ctx: Context) -> KoralResult<()> {
    let shell_str = ctx
        .get::<ShellFlag>()
        .ok_or(KoralError::Validation("Missing shell argument".into()))?;

    let shell = match shell_str.to_lowercase().as_str() {
        "bash" => Shell::Bash,
        "zsh" => Shell::Zsh,
        "fish" => Shell::Fish,
        _ => {
            return Err(KoralError::Validation(format!(
                "Unsupported shell: {}",
                shell_str
            )))
        }
    };

    let app = crate::OpsApp::default();
    let mut out = stdout();
    generate_to(&app, shell, &mut out).map_err(|e| KoralError::IoError(e.to_string()))?;
    out.flush()
        .map_err(|e| KoralError::IoError(e.to_string()))?;

    Ok(())
}

/// Generate man page
#[derive(Default, App, Debug)]
#[app(
    name = "man",
    help = "Generate man page",
    action = man_action
)]
pub struct ManCmd;

fn man_action(_app: &mut ManCmd, _ctx: Context) -> KoralResult<()> {
    let app = crate::OpsApp::default();
    let man = koral::man::generate_man_page(&app, "Jan 2024");
    println!("{}", man);
    Ok(())
}
