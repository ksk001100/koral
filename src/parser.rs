use crate::context::Context;
use crate::error::KoralResult;
use clap::{Arg, ArgAction, Command};
use std::collections::HashMap;

/// Command line argument parser
pub struct Parser {
    known_flags: Vec<crate::flag::FlagDef>,
    strict: bool,
    ignore_required: bool,
}

impl Parser {
    /// Create a new Parser with definitions of known flags.
    pub fn new(flags: Vec<crate::flag::FlagDef>) -> Self {
        Self {
            known_flags: flags,
            strict: false,
            ignore_required: false,
        }
    }

    /// Helper to set strict mode
    pub fn strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    /// Helper to set whether to ignore required flags (e.g. for help)
    pub fn ignore_required(mut self, ignore: bool) -> Self {
        self.ignore_required = ignore;
        self
    }

    /// Parse the provided arguments into a Context.
    pub fn parse<'a>(&self, args: &[String]) -> KoralResult<Context<'a>> {
        // Only allow negative numbers if no short flags are digits
        let has_numeric_flag = self
            .known_flags
            .iter()
            .any(|f| f.short.map(|s| s.is_ascii_digit()).unwrap_or(false));

        let mut cmd = Command::new("koral")
            .no_binary_name(true)
            .allow_negative_numbers(!has_numeric_flag)
            .disable_help_flag(true)
            .disable_version_flag(true);

        if !self.strict {
            cmd = cmd.allow_external_subcommands(true);
        }

        for flag in &self.known_flags {
            cmd = cmd.arg(create_arg(flag, self.ignore_required));
        }

        // Add a catch-all for positionals to allow them in strict mode
        cmd = cmd.arg(
            Arg::new("_positionals")
                .action(ArgAction::Append)
                .num_args(0..)
                .value_parser(clap::builder::StringValueParser::new()),
        );

        let matches_res = cmd.clone().try_get_matches_from(args);

        // Manual recovery for non-strict mode if clap failed or if we want to mimic koral exactly
        let matches = match matches_res {
            Ok(m) => m,
            Err(e) => {
                use clap::error::ErrorKind;
                if !self.strict
                    && (e.kind() == ErrorKind::UnknownArgument
                        || e.kind() == ErrorKind::InvalidSubcommand)
                {
                    // In loose mode, use ignore_errors(true)
                    cmd.ignore_errors(true)
                        .try_get_matches_from(args)
                        .unwrap_or_else(|_| clap::ArgMatches::default())
                } else {
                    return Err(e);
                }
            }
        };

        let mut flags_map: HashMap<String, Option<String>> = HashMap::new();
        let mut positionals = Vec::new();

        // Extract flags_map
        for flag in &self.known_flags {
            let id = if flag.name == "version" {
                "koral_version"
            } else if flag.name == "help" {
                "koral_help"
            } else {
                Box::leak(flag.name.clone().into_boxed_str()) as &'static str
            };

            if flag.takes_value {
                if let Some(val) = matches.get_one::<String>(id) {
                    flags_map.insert(flag.name.clone(), Some(val.clone()));
                } else if let Some(def) = &flag.default_value {
                    flags_map.insert(flag.name.clone(), Some(def.clone()));
                }
            } else {
                if matches.get_flag(id) {
                    flags_map.insert(flag.name.clone(), None);
                }
            }
        }

        // Extract positionals
        if let Some(pos_matches) = matches.get_many::<String>("_positionals") {
            for pos in pos_matches {
                positionals.push(pos.clone());
            }
        }

        // Final legacy positional recovery for non-strict mode
        if !self.strict {
            let mut manual_pos = Vec::new();
            let mut i = 0;
            while i < args.len() {
                let arg = &args[i];
                if arg == "--" {
                    manual_pos.extend(args[i + 1..].iter().cloned());
                    break;
                }

                let mut is_flag = false;
                for flag in &self.known_flags {
                    let long = format!("--{}", flag.long.as_deref().unwrap_or(&flag.name));
                    let short = flag.short.map(|s| format!("-{}", s));
                    if arg == &long || short.as_ref().map(|s| s == arg).unwrap_or(false) {
                        is_flag = true;
                        if flag.takes_value && i + 1 < args.len() {
                            i += 1;
                        }
                        break;
                    }
                }

                if !is_flag {
                    if !manual_pos.contains(arg) {
                        manual_pos.push(arg.clone());
                    }
                }
                i += 1;
            }
            positionals = manual_pos;
        }

        self.apply_defaults(&mut flags_map);

        Ok(Context::new(flags_map, positionals))
    }

    fn apply_defaults(&self, flags_map: &mut HashMap<String, Option<String>>) {
        use crate::provider::{DefaultProvider, EnvProvider, ValueProvider};

        let providers: Vec<Box<dyn ValueProvider>> =
            vec![Box::new(EnvProvider), Box::new(DefaultProvider)];

        for flag in &self.known_flags {
            if !flags_map.contains_key(&flag.name) {
                // Try providers in order
                for provider in &providers {
                    if let Some(val) = provider.get_value(flag) {
                        if flag.takes_value {
                            flags_map.insert(flag.name.clone(), Some(val));
                        } else {
                            // For boolean flags, handle various string representations
                            if val != "0" && val.to_lowercase() != "false" && !val.is_empty() {
                                flags_map.insert(flag.name.clone(), None);
                            } else {
                                flags_map.insert(flag.name.clone(), Some("false".to_string()));
                            }
                        }
                        break;
                    }
                }
            }
        }
    }
}

fn get_styles() -> clap::builder::Styles {
    use clap::builder::styling::{AnsiColor, Effects, Styles};
    Styles::styled()
        .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
        .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
        .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
        .placeholder(AnsiColor::Cyan.on_default())
        .error(AnsiColor::Red.on_default().effects(Effects::BOLD))
        .valid(AnsiColor::Green.on_default().effects(Effects::BOLD))
        .invalid(AnsiColor::Red.on_default().effects(Effects::BOLD))
}

/// Helper to recursively build a clap::Command from an App.
pub fn build_command<T: crate::traits::App + ?Sized>(app: &T) -> Command {
    let flags = app.flags();
    let conflict_v = flags.iter().any(|f| {
        (f.short == Some('V') || f.long.as_deref() == Some("version")) && f.name != "version"
    });
    let conflict_h = flags
        .iter()
        .any(|f| (f.short == Some('h') || f.long.as_deref() == Some("help")) && f.name != "help");

    let mut cmd = Command::new(Box::leak(app.name().to_string().into_boxed_str()) as &'static str)
        .version(Box::leak(app.version().to_string().into_boxed_str()) as &'static str)
        .about(Box::leak(app.description().to_string().into_boxed_str()) as &'static str)
        .styles(get_styles())
        .color(clap::ColorChoice::Always);

    if conflict_v {
        cmd = cmd.disable_version_flag(true);
    }
    if conflict_h {
        cmd = cmd.disable_help_flag(true);
    }

    for flag in &flags {
        if !conflict_v && flag.name == "version" {
            continue;
        }
        if !conflict_h && flag.name == "help" {
            continue;
        }
        cmd = cmd.arg(create_arg(flag, false));
    }

    for sub in app.subcommands() {
        cmd = cmd.subcommand(build_command_from_def(&sub));
    }

    cmd
}

fn build_command_from_def(def: &crate::command::CommandDef) -> Command {
    let conflict_v = def.flags.iter().any(|f| {
        (f.short == Some('V') || f.long.as_deref() == Some("version")) && f.name != "version"
    });
    let conflict_h = def
        .flags
        .iter()
        .any(|f| (f.short == Some('h') || f.long.as_deref() == Some("help")) && f.name != "help");

    let mut cmd = Command::new(Box::leak(def.name.clone().into_boxed_str()) as &'static str)
        .about(Box::leak(def.description.clone().into_boxed_str()) as &'static str)
        .styles(get_styles())
        .color(clap::ColorChoice::Always);

    if conflict_v {
        cmd = cmd.disable_version_flag(true);
    }
    if conflict_h {
        cmd = cmd.disable_help_flag(true);
    }

    for flag in &def.flags {
        if !conflict_v && flag.name == "version" {
            continue;
        }
        if !conflict_h && flag.name == "help" {
            continue;
        }
        cmd = cmd.arg(create_arg(flag, false));
    }

    for sub in &def.subcommands {
        cmd = cmd.subcommand(build_command_from_def(sub));
    }

    cmd
}

fn create_arg(flag: &crate::flag::FlagDef, ignore_required: bool) -> Arg {
    let id = if flag.name == "version" {
        "koral_version"
    } else if flag.name == "help" {
        "koral_help"
    } else {
        Box::leak(flag.name.clone().into_boxed_str()) as &'static str
    };

    let mut arg = Arg::new(id).help(Box::leak(flag.help.clone().into_boxed_str()) as &'static str);

    if let Some(long) = &flag.long {
        arg = arg.long(Box::leak(long.clone().into_boxed_str()) as &'static str);
    } else {
        arg = arg.long(Box::leak(flag.name.clone().into_boxed_str()) as &'static str);
    }

    if let Some(s) = flag.short {
        arg = arg.short(s);
    }

    for alias in &flag.aliases {
        arg = arg.alias(Box::leak(alias.clone().into_boxed_str()) as &'static str);
    }

    if flag.takes_value {
        arg = arg.action(ArgAction::Set);
        if let Some(vn) = &flag.value_name {
            let vn_static: &'static str = Box::leak(vn.clone().into_boxed_str());
            arg = arg.value_name(vn_static);

            // Replicate koral's logic for completion hints
            let vn_upper = vn.to_uppercase();
            if vn_upper.contains("FILE") || vn_upper.contains("PATH") {
                arg = arg.value_hint(clap::ValueHint::FilePath);
            } else if vn_upper.contains("DIR") {
                arg = arg.value_hint(clap::ValueHint::DirPath);
            }
        }
    } else {
        arg = arg.action(ArgAction::SetTrue);
    }

    if let Some(env) = &flag.env {
        arg = arg.env(Box::leak(env.clone().into_boxed_str()) as &'static str);
    }

    if flag.required && !ignore_required {
        arg = arg.required(true);
    }

    if let Some(heading) = &flag.help_heading {
        arg = arg.help_heading(Box::leak(heading.clone().into_boxed_str()) as &'static str);
    }

    if let Some(validator) = flag.validator {
        arg = arg.value_parser(move |s: &str| -> Result<String, String> {
            validator(s).map(|_| s.to_string())
        });
    }

    arg
}

/// Helper function to validate required flags externally.
pub fn validate_required_flags(
    flags: &[crate::flag::FlagDef],
    flags_map: &HashMap<String, Option<String>>,
) -> KoralResult<()> {
    for flag in flags {
        if flag.required && !flags_map.contains_key(&flag.name) {
            return Err(clap::Error::raw(
                clap::error::ErrorKind::MissingRequiredArgument,
                format!("Required flag '--{}' is missing", flag.name),
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flag::FlagDef;

    #[test]
    fn test_validate_required_flags() {
        let req_flag = FlagDef {
            name: "req".to_string(),
            required: true,
            short: None,
            long: None,
            help: "".to_string(),
            takes_value: true,
            default_value: None,
            env: None,
            validator: None,
            aliases: vec![],
            value_name: None,
            help_heading: None,
        };
        let opt_flag = FlagDef {
            name: "opt".to_string(),
            required: false,
            short: None,
            long: None,
            help: "".to_string(),
            takes_value: true,
            default_value: None,
            env: None,
            validator: None,
            aliases: vec![],
            value_name: None,
            help_heading: None,
        };

        let flags = vec![req_flag, opt_flag];

        // Case 1: All present
        let mut map = HashMap::new();
        map.insert("req".to_string(), Some("val".to_string()));
        assert!(validate_required_flags(&flags, &map).is_ok());

        // Case 2: Required missing
        let mut map2 = HashMap::new();
        map2.insert("opt".to_string(), Some("val".to_string()));
        let err = validate_required_flags(&flags, &map2);
        assert!(err.is_err());
        assert_eq!(
            err.unwrap_err().kind(),
            clap::error::ErrorKind::MissingRequiredArgument
        );

        // Case 3: Required present, Optional missing (OK)
        let mut map3 = HashMap::new();
        map3.insert("req".to_string(), Some("val".to_string()));
        assert!(validate_required_flags(&flags, &map3).is_ok());
    }
}
