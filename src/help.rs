use crate::traits::App;

/// Generate help message for the application.
pub fn generate_help<T: App + ?Sized>(app: &T) -> String {
    let mut out = String::new();

    out.push_str(&format!("Usage: {} [options] [command]\n", app.name()));
    let desc = app.description();
    if !desc.is_empty() {
        out.push_str(&format!("{}\n", desc));
    }

    out.push_str("\nOptions:\n");

    struct HelpItem {
        name: String,
        desc: String,
    }

    let mut items = Vec::new();

    // Built-in flags
    items.push(HelpItem {
        name: "--version".to_string(),
        desc: "Show version information".to_string(),
    });
    items.push(HelpItem {
        name: "--help, -h".to_string(),
        desc: "Show help information".to_string(),
    });

    let mut flags = app.flags();
    // Sort flags by name for consistent output
    flags.sort_by(|a, b| a.name.cmp(&b.name));

    for flag in flags {
        let mut name_part = format!("--{}", flag.name);
        if let Some(s) = flag.short {
            name_part.push_str(&format!(", -{}", s));
        }

        if flag.takes_value {
            name_part.push_str(" <value>");
        }

        if !flag.aliases.is_empty() {
            name_part.push_str(" (aliases: ");
            name_part.push_str(
                &flag
                    .aliases
                    .iter()
                    .map(|a| format!("--{}", a))
                    .collect::<Vec<_>>()
                    .join(", "),
            );
            name_part.push(')');
        }

        items.push(HelpItem {
            name: name_part,
            desc: flag.help.clone(),
        });
    }

    // Calculate max width for alignment
    let max_width = items.iter().map(|i| i.name.len()).max().unwrap_or(0);
    let padding = 2;

    for item in items {
        let pad_len = max_width.saturating_sub(item.name.len()) + padding;
        let pad = " ".repeat(pad_len);
        out.push_str(&format!("  {}{}{}\n", item.name, pad, item.desc));
    }

    let mut subs = app.subcommands();
    if !subs.is_empty() {
        out.push_str("\nCommands:\n");
        // Sort subcommands
        subs.sort_by(|a, b| a.name.cmp(&b.name));

        let max_sub_width = subs.iter().map(|s| s.name.len()).max().unwrap_or(0);

        for sub in subs {
            let pad_len = max_sub_width.saturating_sub(sub.name.len()) + padding;
            let pad = " ".repeat(pad_len);
            let mut line = format!("  {}{}{}", sub.name, pad, sub.description);
            if !sub.aliases.is_empty() {
                line.push_str(&format!(" (aliases: {})", sub.aliases.join(", ")));
            }
            out.push_str(&format!("{}\n", line));
        }
    }

    // Remove trailing newline if any? standard println! adds one.
    // Let's keep one trailing newline.

    out
}
