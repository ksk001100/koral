use crate::traits::App;
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

/// Generate completion script for the given app and shell
pub fn generate_to<W: Write>(app: &impl App, shell: Shell, buf: &mut W) -> io::Result<()> {
    match shell {
        Shell::Bash => generate_bash(app, buf),
        Shell::Zsh => generate_zsh(app, buf),
        Shell::Fish => generate_fish(app, buf),
    }
}

fn generate_bash<W: Write>(app: &impl App, buf: &mut W) -> io::Result<()> {
    let name = app.name();
    let mut opts = Vec::new();

    // Add flags
    for flag in app.flags() {
        opts.push(format!("--{}", flag.name));
        if let Some(s) = flag.short {
            opts.push(format!("-{}", s));
        }
    }

    // Add subcommands
    for cmd in app.subcommands() {
        opts.push(cmd.name.clone());
    }

    let opts_str = opts.join(" ");

    writeln!(buf, "_{}_completion() {{", name)?;
    writeln!(buf, "    local cur prev opts")?;
    writeln!(buf, "    COMPREPLY=()")?;
    writeln!(buf, "    cur=\"${{COMP_WORDS[COMP_CWORD]}}\"")?;
    writeln!(buf, "    prev=\"${{COMP_WORDS[COMP_CWORD-1]}}\"")?;
    writeln!(buf, "    opts=\"{}\"", opts_str)?;
    writeln!(buf)?;
    writeln!(buf, "    if [[ ${{cur}} == -* ]]; then")?;
    writeln!(
        buf,
        "        COMPREPLY=( $(compgen -W \"${{opts}}\" -- ${{cur}}) )"
    )?;
    writeln!(buf, "        return 0")?;
    writeln!(buf, "    fi")?;
    writeln!(buf)?;
    writeln!(buf, "    # Basic subcommand completion (stateless for now)")?;
    writeln!(
        buf,
        "    COMPREPLY=( $(compgen -W \"${{opts}}\" -- ${{cur}}) )"
    )?;
    writeln!(buf, "}}")?;
    writeln!(buf)?;
    writeln!(buf, "complete -F _{}_completion {}", name, name)?;

    Ok(())
}

fn generate_zsh<W: Write>(app: &impl App, buf: &mut W) -> io::Result<()> {
    let name = app.name();

    writeln!(buf, "#compdef {}", name)?;
    writeln!(buf)?;
    writeln!(buf, "_{}() {{", name)?;
    writeln!(buf, "    local -a args")?;
    writeln!(buf, "    args=(")?;

    // Flags
    for flag in app.flags() {
        let help = escape_help(&flag.help);
        let mut arg_spec = String::new();

        if flag.takes_value {
            let vname = flag.value_name.as_deref().unwrap_or("value");
            let v_upper = vname.to_uppercase();

            let action = if v_upper.contains("FILE") || v_upper.contains("PATH") {
                ":_files"
            } else if v_upper.contains("DIR") {
                ":_files -/"
            } else {
                // Default: just show value name as message, no specific completion
                ""
            };
            arg_spec = format!(":{}{}", vname, action);
        }

        if let Some(s) = flag.short {
            writeln!(buf, "        '-{}[{}]{}'", s, help, arg_spec)?;
        }
        writeln!(buf, "        '--{}[{}]{}'", flag.name, help, arg_spec)?;
    }

    // Subcommands
    // Zsh handles subcommands nicely if we define them, but for strict top-level
    // we can just add them to the list or use a simpler approach.
    // For now, let's treat them as positional arguments that are completions.
    for cmd in app.subcommands() {
        let desc = escape_help(&cmd.description);
        writeln!(buf, "        '{}:{}'", cmd.name, desc)?;
    }

    writeln!(buf, "    )")?;
    writeln!(buf, "    _arguments -s $args")?;
    writeln!(buf, "}}")?;

    Ok(())
}

fn generate_fish<W: Write>(app: &impl App, buf: &mut W) -> io::Result<()> {
    let name = app.name();

    // Disable file completion by default for better UX with subcommands?
    // Usually keep it, but fish requires explicit -f to disable.
    // Let's keep file completion enabled unless otherwise specified.

    // Flags
    for flag in app.flags() {
        let help = escape_help(&flag.help);
        let mut line = format!("complete -c {}", name);
        if let Some(s) = flag.short {
            line.push_str(&format!(" -s {}", s));
        }
        line.push_str(&format!(" -l {}", flag.name));
        line.push_str(&format!(" -d '{}'", help));

        if flag.takes_value {
            line.push_str(" -r"); // Require argument
                                  // If we had a way to specify value completion (e.g. files), we would add it here.
        }

        writeln!(buf, "{}", line)?;
    }

    // Subcommands
    for cmd in app.subcommands() {
        let desc = escape_help(&cmd.description);
        // Fish subcommands command: complete -c todo -a "add" -d "Add logic"
        // This is a simplification; fish doesn't strictly distinguish subcommands in `complete`.
        // It treats them as arguments.
        // To prevent file completion for subcommands, we might want -f -n "__fish_use_subcommand"
        writeln!(buf, "complete -c {} -a '{}' -d '{}'", name, cmd.name, desc)?;
    }

    Ok(())
}

fn escape_help(s: &str) -> String {
    s.replace("'", "'\\''")
}
