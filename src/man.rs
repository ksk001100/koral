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
    let subcommands = app.subcommands();
    if !subcommands.is_empty() {
        out.push_str(".SH COMMANDS\n");
        let mut sorted_subs = subcommands.clone();
        sorted_subs.sort_by(|a, b| a.name.cmp(&b.name));

        for sub in sorted_subs {
            out.push_str(".TP\n");
            out.push_str(&format!("\\fB{}\\fR\n", sub.name));
            let safe_desc = sub.description.replace('-', "\\-");
            out.push_str(&format!("{}\n", safe_desc));
        }
    }

    out
}
