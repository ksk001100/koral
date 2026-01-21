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

    writeln!(buf, "_{}_completion() {{", name)?;
    writeln!(buf, "    local cur prev first word")?;
    writeln!(
        buf,
        "    if type -t _get_comp_words_by_ref >/dev/null 2>&1; then"
    )?;
    writeln!(
        buf,
        "        _get_comp_words_by_ref -n : -w words -i cword cur prev"
    )?;
    writeln!(buf, "    else")?;
    writeln!(buf, "        cur=\"${{COMP_WORDS[COMP_CWORD]}}\"")?;
    writeln!(buf, "        prev=\"${{COMP_WORDS[COMP_CWORD-1]}}\"")?;
    writeln!(buf, "        words=(\"${{COMP_WORDS[@]}}\")")?;
    writeln!(buf, "        cword=$COMP_CWORD")?;
    writeln!(buf, "    fi")?;
    writeln!(buf)?;
    writeln!(buf, "    # Find the current subcommand context")?;
    writeln!(buf, "    local i")?;
    writeln!(buf, "    local cmd=\"{}\"", name)?;
    writeln!(buf, "    for (( i=1; i < cword; i++ )); do")?;
    writeln!(buf, "        local word=\"${{words[i]}}\"")?;
    writeln!(buf, "        if [[ \"$word\" != -* ]]; then")?;
    writeln!(buf, "            cmd=\"${{cmd}}_${{word}}\"")?;
    writeln!(buf, "        fi")?;
    writeln!(buf, "    done")?;
    writeln!(buf)?;
    writeln!(buf, "    case \"$cmd\" in")?;

    // Collect all commands (root + recursive)
    let root = crate::command::CommandDef {
        name: name.to_string(),
        description: app.description().to_string(),
        aliases: vec![],
        subcommands: app.subcommands(),
        flags: app.flags(),
    };

    // Helper to linearize commands
    fn collect_cmds(
        parent_prefix: &str,
        cmd: &crate::command::CommandDef,
        acc: &mut Vec<(String, crate::command::CommandDef)>,
    ) {
        let current_prefix = if parent_prefix.is_empty() {
            cmd.name.clone()
        } else {
            format!("{}_{}", parent_prefix, cmd.name)
        };
        acc.push((current_prefix.clone(), cmd.clone()));
        for sub in &cmd.subcommands {
            collect_cmds(&current_prefix, sub, acc);
        }
    }

    let mut all_cmds = Vec::new();
    // Root is special, it's just name
    all_cmds.push((name.to_string(), root.clone()));
    for sub in &root.subcommands {
        collect_cmds(name, sub, &mut all_cmds);
    }

    for (prefix, cmd) in all_cmds {
        let mut opts = Vec::new();
        for flag in &cmd.flags {
            opts.push(format!("--{}", flag.name));
            if let Some(s) = flag.short {
                opts.push(format!("-{}", s));
            }
        }
        for sub in &cmd.subcommands {
            opts.push(sub.name.clone());
        }

        writeln!(buf, "        {})", prefix)?;
        writeln!(buf, "            opts=\"{}\"", opts.join(" "))?;
        writeln!(
            buf,
            "            COMPREPLY=( $(compgen -W \"${{opts}}\" -- ${{cur}}) )"
        )?;
        writeln!(buf, "            ;;")?;
    }

    writeln!(buf, "        *)")?;
    writeln!(buf, "            ;;")?;
    writeln!(buf, "    esac")?;
    writeln!(buf, "}}")?;
    writeln!(buf, "complete -F _{}_completion {}", name, name)?;

    Ok(())
}

fn generate_zsh<W: Write>(app: &impl App, buf: &mut W) -> io::Result<()> {
    let name = app.name();
    writeln!(buf, "#compdef {}", name)?;

    let root = crate::command::CommandDef {
        name: name.to_string(),
        description: app.description().to_string(),
        aliases: vec![],
        subcommands: app.subcommands(),
        flags: app.flags(),
    };

    // Recursive function generator
    fn write_zsh_func<W: Write>(
        buf: &mut W,
        cmd: &crate::command::CommandDef,
        func_name: &str,
    ) -> io::Result<()> {
        writeln!(buf, "function {} {{", func_name)?;
        writeln!(buf, "    local -a args")?;
        writeln!(buf, "    args=(")?;

        // Flags
        for flag in &cmd.flags {
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
        if !cmd.subcommands.is_empty() {
            writeln!(buf, "        '1: :_subcommands'")?;
            writeln!(buf, "        '*:: :->args'")?;
        }
        writeln!(buf, "    )")?;

        writeln!(buf, "    _arguments -s $args")?;

        if !cmd.subcommands.is_empty() {
            writeln!(buf, "    case $state in")?;
            writeln!(buf, "        args)")?;
            writeln!(buf, "            case $words[1] in")?;
            for sub in &cmd.subcommands {
                let sub_func = format!("{}_{}", func_name, sub.name);
                writeln!(buf, "                {}) {} ;;", sub.name, sub_func)?;
            }
            writeln!(buf, "            esac")?;
            writeln!(buf, "            ;;")?;
            writeln!(buf, "    esac")?;

            // Define local function to list subcommands
            writeln!(buf, "    function _subcommands {{")?;
            writeln!(buf, "        local -a commands")?;
            writeln!(buf, "        commands=(")?;
            for sub in &cmd.subcommands {
                writeln!(
                    buf,
                    "            '{}:{}'",
                    sub.name,
                    escape_help(&sub.description)
                )?;
            }
            writeln!(buf, "        )")?;
            writeln!(buf, "        _describe -t commands 'commands' commands")?;
            writeln!(buf, "    }}")?;
        }

        writeln!(buf, "}}")?;

        // Recursively write functions for children
        for sub in &cmd.subcommands {
            write_zsh_func(buf, sub, &format!("{}_{}", func_name, sub.name))?;
        }

        Ok(())
    }

    write_zsh_func(buf, &root, &format!("_{}", name))?;

    Ok(())
}

fn generate_fish<W: Write>(app: &impl App, buf: &mut W) -> io::Result<()> {
    let name = app.name();
    let root = crate::command::CommandDef {
        name: name.to_string(),
        description: app.description().to_string(),
        aliases: vec![],
        subcommands: app.subcommands(),
        flags: app.flags(),
    };

    fn write_fish_cmd<W: Write>(
        buf: &mut W,
        root_name: &str,
        cmd: &crate::command::CommandDef,
        path: &[String],
    ) -> io::Result<()> {
        // Condition: we are inside this command path
        // For root 'app', condition is always true (or implies NOT inside any other subcommand if we were implicit, but fish completions are additive)
        // With Fish, we generally exclude completion of a subcommand if we possess seen it.
        // And we include flags of current subcommand if we have seen it but NOT seen nested subcommands.

        // Helper to construct "seen subcommand" logic
        let condition = if path.is_empty() {
            // Top level: NOT seen any subcommand
            format!(
                "not __fish_seen_subcommand_from {}",
                cmd.subcommands
                    .iter()
                    .map(|s| s.name.as_str())
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        } else {
            // Nested: Seen path[-1] AND not seen any children
            // But path[-1] is just name.
            // We need to trace full path?
            // Fish's __fish_seen_subcommand_from checks *anywhere* in line.
            // This is ambiguous if same subcommand name exists in multiple places.
            // For strict correctness we might need a custom function.
            // For now, let's use the standard approach:
            // To complete flags for `app sub`:
            // -n "__fish_seen_subcommand_from sub"

            format!("__fish_seen_subcommand_from {}", cmd.name)
        };

        // Write flags
        for flag in &cmd.flags {
            let help = escape_help(&flag.help);
            let mut line = format!("complete -c {}", root_name);

            if !path.is_empty() {
                // To avoid conflict (flag name collision), strict conditions are needed.
                // Simple __fish_seen_subcommand_from is often enough for simple apps.
                // But for `app sub1 --flag` vs `app sub2 --flag`, both see their respective subcommand.
                line.push_str(&format!(" -n '{}'", condition));
            } else {
                line.push_str(" -n \"not __fish_seen_subcommand_from ");
                for sub in &cmd.subcommands {
                    line.push_str(&format!("{} ", sub.name));
                }
                line.push_str("\"");
            }

            if let Some(s) = flag.short {
                line.push_str(&format!(" -s {}", s));
            }
            line.push_str(&format!(" -l {}", flag.name));
            line.push_str(&format!(" -d '{}'", help));
            if flag.takes_value {
                line.push_str(" -r");
            }
            writeln!(buf, "{}", line)?;
        }

        // Write subcommands
        for sub in &cmd.subcommands {
            let desc = escape_help(&sub.description);
            let mut line = format!("complete -c {} -a '{}' -d '{}'", root_name, sub.name, desc);
            // Show this subcommand ONLY if we are in current command context
            line.push_str(&format!(" -n '{}'", condition));
            writeln!(buf, "{}", line)?;

            // Recurse
            let mut new_path = path.to_vec();
            new_path.push(sub.name.clone());
            write_fish_cmd(buf, root_name, sub, &new_path)?;
        }
        Ok(())
    }

    write_fish_cmd(buf, name, &root, &[])?;

    Ok(())
}

fn escape_help(s: &str) -> String {
    s.replace("'", "'\\''")
}
