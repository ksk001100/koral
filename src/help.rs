use crate::traits::App;

/// Generate help message for the application with ANSI colors.
pub fn generate_help<T: App + ?Sized>(app: &T) -> String {
    let mut out = String::new();
    let header_style = anstyle::Style::new().bold().underline();
    let literal_style = anstyle::Style::new()
        .bold()
        .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Cyan)));
    let title_style = anstyle::Style::new()
        .bold()
        .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow)));

    out.push_str(&format!(
        "{header_style}Usage:{header_style:#} {title_style}{}{title_style:#} [options] [command]\n",
        app.name()
    ));
    // Get terminal width
    let term_width = std::env::var("COLUMNS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);

    // Helper to wrap text
    let wrap_desc = |text: &str, indent_len: usize| -> String {
        // If indent is too large relative to term_width, just print simple
        if indent_len >= term_width {
            return text.to_string();
        }
        let avail = term_width - indent_len;
        if avail < 10 {
            // Too narrow
            return text.to_string();
        }

        let mut out = String::new();
        let mut line_len = 0;

        for word in text.split_whitespace() {
            let wlen = word.len();
            if line_len + 1 + wlen > avail {
                // New line
                out.push('\n');
                out.push_str(&" ".repeat(indent_len));
                out.push_str(word);
                line_len = wlen;
            } else {
                if line_len > 0 {
                    out.push(' ');
                    line_len += 1;
                }
                out.push_str(word);
                line_len += wlen;
            }
        }
        out
    };

    let desc = app.description();
    if !desc.is_empty() {
        // Wrap description with no indent
        let wrapped = wrap_desc(&desc, 0);
        out.push_str(&format!("{}\n", wrapped));
    }

    struct HelpItem {
        display: String,
        real_len: usize,
        desc: String,
    }

    use std::collections::BTreeMap;
    let mut groups: BTreeMap<Option<String>, Vec<HelpItem>> = BTreeMap::new();

    let mut flags = app.flags();
    // Sort flags by name
    flags.sort_by(|a, b| a.name.cmp(&b.name));

    let h_overridden = flags.iter().any(|f| f.short == Some('h'));
    let help_name_display = if h_overridden {
        format!("{literal_style}--help{literal_style:#}")
    } else {
        format!("{literal_style}--help{literal_style:#}, {literal_style}-h{literal_style:#}")
    };
    let help_len = if h_overridden { 6 } else { 10 };

    // Built-in flags (Default group)
    {
        let default_group = groups.entry(None).or_default();
        default_group.push(HelpItem {
            display: help_name_display,
            real_len: help_len,
            desc: "Show help information".to_string(),
        });
    }
    // Version is handled by standard flags provided by App trait now.

    for flag in flags {
        if flag.name == "version" {
            let already_has_version = groups
                .get(&None)
                .map(|v| v.iter().any(|i| i.display.contains("version")))
                .unwrap_or(false);
            if already_has_version {
                continue;
            }
        }

        let mut name_part_display = format!("{literal_style}--{}{literal_style:#}", flag.name);
        // Base len: "--" + name
        let mut name_part_len = 2 + flag.name.len();

        if let Some(s) = flag.short {
            name_part_display.push_str(&format!(", {literal_style}-{}{literal_style:#}", s));
            name_part_len += 4; // ", -s"
        }

        if flag.takes_value {
            let vname = flag.value_name.as_deref().unwrap_or("value");
            name_part_display.push_str(&format!(" <{}>", vname));
            name_part_len += 3 + vname.len(); // " <vname>"
        }

        if !flag.aliases.is_empty() {
            name_part_display.push_str(" (aliases: ");
            name_part_display.push_str(
                &flag
                    .aliases
                    .iter()
                    .map(|a| format!("{literal_style}--{}{literal_style:#}", a))
                    .collect::<Vec<_>>()
                    .join(", "),
            );
            name_part_display.push(')');
            // Update length manually for aliases
            // " (aliases: --a, --b)"
            // " (aliases: " = 11
            // each alias: "--" + name = 2 + len
            // join: ", " = 2
            // ")" = 1
            let aliases_len: usize = flag.aliases.iter().map(|a| 2 + a.len()).sum();
            let joins = if flag.aliases.len() > 1 {
                (flag.aliases.len() - 1) * 2
            } else {
                0
            };
            name_part_len += 11 + aliases_len + joins + 1;
        }

        groups
            .entry(flag.help_heading.clone())
            .or_default()
            .push(HelpItem {
                display: name_part_display,
                real_len: name_part_len,
                desc: flag.help.clone(),
            });
    }

    // Sort items within groups (insertion order mostly respected, but flags were sorted by name)

    // Calculate max width for alignment
    // For visual clarity, cap alignment width.
    // If a flag string is too long, we might want to put description on next line,
    // but for now let's stick to simple tabular with wrapping.
    let max_width = groups
        .values()
        .flat_map(|items| items.iter().map(|i| i.real_len))
        .max()
        .unwrap_or(0);

    let padding = 4;
    let indent = max_width + 2 + padding; // "  " + name + pad

    // Output groups
    // 1. None (Options) first
    if let Some(items) = groups.get(&None) {
        out.push_str("\nOptions:\n");
        for item in items {
            let pad_len = max_width.saturating_sub(item.real_len) + padding;
            let pad = " ".repeat(pad_len);
            let desc = wrap_desc(&item.desc, indent);
            out.push_str(&format!("  {}{}{}\n", item.display, pad, desc));
        }
    }

    // 2. Named groups
    for (heading, items) in &groups {
        if let Some(h) = heading {
            out.push_str(&format!("\n{}:\n", h));
            for item in items {
                let pad_len = max_width.saturating_sub(item.real_len) + padding;
                let pad = " ".repeat(pad_len);
                let desc = wrap_desc(&item.desc, indent);
                out.push_str(&format!("  {}{}{}\n", item.display, pad, desc));
            }
        }
    }

    let mut subs = app.subcommands();
    if !subs.is_empty() {
        out.push_str("\nCommands:\n");
        subs.sort_by(|a, b| a.name.cmp(&b.name));

        let max_sub_width = subs.iter().map(|s| s.name.len()).max().unwrap_or(0);
        let sub_indent = max_sub_width + 2 + padding;

        for sub in subs {
            let pad_len = max_sub_width.saturating_sub(sub.name.len()) + padding;
            let pad = " ".repeat(pad_len);
            let name_colored = format!("{literal_style}{}{literal_style:#}", sub.name);

            let mut desc_full = sub.description;
            if !sub.aliases.is_empty() {
                desc_full.push_str(&format!(" (aliases: {})", sub.aliases.join(", ")));
            }
            let desc = wrap_desc(&desc_full, sub_indent);

            out.push_str(&format!("  {}{}{}\n", name_colored, pad, desc));
        }
    }

    out
}
