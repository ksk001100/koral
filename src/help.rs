use crate::traits::App;

/// Generate help message for the application.
pub fn generate_help<T: App + ?Sized>(app: &T) -> String {
    let mut out = String::new();

    out.push_str(&format!("Usage: {} [options] [command]\n", app.name()));
    let desc = app.description();
    if !desc.is_empty() {
        out.push_str(&format!("{}\n", desc));
    }

    struct HelpItem {
        name: String,
        desc: String,
    }

    use std::collections::BTreeMap;
    let mut groups: BTreeMap<Option<String>, Vec<HelpItem>> = BTreeMap::new();

    let mut flags = app.flags();
    // Sort flags by name
    flags.sort_by(|a, b| a.name.cmp(&b.name));

    let h_overridden = flags.iter().any(|f| f.short == Some('h'));
    let help_name = if h_overridden {
        "--help".to_string()
    } else {
        "--help, -h".to_string()
    };

    // Built-in flags (Default group)
    let default_group = groups.entry(None).or_default();
    default_group.push(HelpItem {
        name: "--version".to_string(),
        desc: "Show version information".to_string(),
    });
    default_group.push(HelpItem {
        name: help_name,
        desc: "Show help information".to_string(),
    });

    for flag in flags {
        let mut name_part = format!("--{}", flag.name);
        if let Some(s) = flag.short {
            name_part.push_str(&format!(", -{}", s));
        }

        if flag.takes_value {
            let vname = flag.value_name.as_deref().unwrap_or("value");
            name_part.push_str(&format!(" <{}>", vname));
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

        groups
            .entry(flag.help_heading.clone())
            .or_default()
            .push(HelpItem {
                name: name_part,
                desc: flag.help.clone(),
            });
    }

    // Calculate max width across ALL items for consistent alignment?
    // Or per section? Usually per section is fine or global.
    // Let's do global alignment for neatness.
    let max_width = groups
        .values()
        .flat_map(|items| items.iter().map(|i| i.name.len()))
        .max()
        .unwrap_or(0);
    let padding = 2;

    // Output groups
    // 1. None (Options) first
    if let Some(items) = groups.get(&None) {
        out.push_str("\nOptions:\n");
        for item in items {
            let pad_len = max_width.saturating_sub(item.name.len()) + padding;
            let pad = " ".repeat(pad_len);
            out.push_str(&format!("  {}{}{}\n", item.name, pad, item.desc));
        }
    }

    // 2. Named groups
    for (heading, items) in &groups {
        if let Some(h) = heading {
            out.push_str(&format!("\n{}:\n", h));
            for item in items {
                let pad_len = max_width.saturating_sub(item.name.len()) + padding;
                let pad = " ".repeat(pad_len);
                out.push_str(&format!("  {}{}{}\n", item.name, pad, item.desc));
            }
        }
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
