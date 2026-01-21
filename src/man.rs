use crate::traits::App;

/// Generate ROFF man page for the application.
/// `date` should be in format "Month Year" or "YYYY-MM-DD".
pub fn generate_man_page<T: App + ?Sized>(app: &T, date: &str) -> String {
    let name = app.name().to_uppercase();
    let mut out = String::new();

    // Header
    // .TH name section date source manual
    out.push_str(&format!(
        ".TH \"{}\" 1 \"{}\" \"Koral\" \"User Commands\"\n",
        name, date
    ));

    // NAME
    out.push_str(".SH NAME\n");
    out.push_str(&format!("{} \\- {}\n", app.name(), app.description()));

    // SYNOPSIS
    out.push_str(".SH SYNOPSIS\n");
    out.push_str(&format!(".B {}\n", app.name()));
    out.push_str("[\\fIOPTIONS\\fR] [\\fICOMMAND\\fR]\n");

    // DESCRIPTION
    out.push_str(".SH DESCRIPTION\n");
    out.push_str(&format!("{}\n", app.description()));

    // OPTIONS
    let flags = app.flags();
    if !flags.is_empty() {
        out.push_str(".SH OPTIONS\n");
        let mut sorted_flags = flags.clone();
        sorted_flags.sort_by(|a, b| a.name.cmp(&b.name));

        for flag in sorted_flags {
            out.push_str(".TP\n");
            let mut line = String::new();
            if let Some(s) = flag.short {
                line.push_str(&format!("\\fB-{}\\fR, ", s));
            }
            line.push_str(&format!("\\fB--{}\\fR", flag.name));
            if flag.takes_value {
                let vname = flag.value_name.as_deref().unwrap_or("value");
                line.push_str(&format!(" \\fI{}\\fR", vname));
            }
            out.push_str(&format!("{}\n", line));
            // Basic escaping for hyphen in description
            let safe_help = flag.help.replace('-', "\\-");
            out.push_str(&format!("{}\n", safe_help));
        }
    }

    // COMMANDS
    // Flatten subcommands
    let root = crate::command::CommandDef {
        name: app.name().to_string(),
        description: app.description().to_string(),
        aliases: vec![],
        subcommands: app.subcommands(),
        flags: vec![], // handled separately
    };

    fn collect_subs(
        prefix: &str,
        cmd: &crate::command::CommandDef,
        acc: &mut Vec<(String, crate::command::CommandDef)>,
    ) {
        let current_name = if prefix.is_empty() {
            cmd.name.clone()
        } else {
            format!("{} {}", prefix, cmd.name)
        };

        acc.push((current_name.clone(), cmd.clone()));

        for sub in &cmd.subcommands {
            collect_subs(&current_name, sub, acc);
        }
    }

    let mut all_subs = Vec::new();
    for sub in &root.subcommands {
        collect_subs("", sub, &mut all_subs);
    }

    if !all_subs.is_empty() {
        out.push_str(".SH COMMANDS\n");
        // Sort by full name
        all_subs.sort_by(|a, b| a.0.cmp(&b.0));

        for (full_name, sub) in &all_subs {
            out.push_str(".TP\n");
            out.push_str(&format!("\\fB{}\\fR\n", full_name));
            let safe_desc = sub.description.replace('-', "\\-");
            out.push_str(&format!("{}\n", safe_desc));
        }
    }

    // NESTED OPTIONS
    // If any subcommand has flags, list them possibly under a separate header or subsection?
    // Man pages usually don't mix all subcommand flags.
    // But since we are generating one page, let's try to group them.

    let subs_with_flags = all_subs
        .iter()
        .filter(|(_, cmd)| !cmd.flags.is_empty())
        .collect::<Vec<_>>();
    if !subs_with_flags.is_empty() {
        out.push_str(".SH SUBCOMMAND OPTIONS\n");
        for (full_name, cmd) in subs_with_flags {
            out.push_str(&format!(".SS Options for \\fB{}\\fR\n", full_name));
            let mut sorted_flags = cmd.flags.clone();
            sorted_flags.sort_by(|a, b| a.name.cmp(&b.name));

            for flag in sorted_flags {
                out.push_str(".TP\n");
                let mut line = String::new();
                if let Some(s) = flag.short {
                    line.push_str(&format!("\\fB-{}\\fR, ", s));
                }
                line.push_str(&format!("\\fB--{}\\fR", flag.name));
                if flag.takes_value {
                    let vname = flag.value_name.as_deref().unwrap_or("value");
                    line.push_str(&format!(" \\fI{}\\fR", vname));
                }
                out.push_str(&format!("{}\n", line));
                let safe_help = flag.help.replace('-', "\\-");
                out.push_str(&format!("{}\n", safe_help));
            }
        }
    }

    out
}
