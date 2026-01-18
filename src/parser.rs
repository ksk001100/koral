use crate::context::Context;
use crate::error::{KoralError, KoralResult};
use std::collections::HashMap;

/// Command line argument parser
pub struct Parser {
    known_flags: Vec<crate::flag::FlagDef>,
    strict: bool,
}

impl Parser {
    /// Create a new Parser with definitions of known flags.
    pub fn new(flags: Vec<crate::flag::FlagDef>) -> Self {
        Self {
            known_flags: flags,
            strict: false,
        }
    }

    /// Helper to set strict mode
    pub fn strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    /// Parse the provided arguments into a Context.
    pub fn parse<'a>(&self, args: &[String]) -> KoralResult<Context<'a>> {
        let mut flags_map: HashMap<String, Option<String>> = HashMap::new();
        let mut positionals: Vec<String> = Vec::new();

        let mut iter = args.iter();
        while let Some(arg) = iter.next() {
            if arg.starts_with("--") {
                // Long flag
                let arg_content = arg.trim_start_matches("--");

                // Check if it's key=value
                let (name_part, value_part) = if let Some(idx) = arg_content.find('=') {
                    (&arg_content[..idx], Some(&arg_content[idx + 1..]))
                } else {
                    (arg_content, None)
                };

                // Find matching flag
                let mut matched_flag: Option<&crate::flag::FlagDef> = None;
                for flag in &self.known_flags {
                    let flag_long = flag.long.as_deref().unwrap_or(&flag.name);
                    if flag_long == name_part || flag.aliases.iter().any(|a| a == name_part) {
                        matched_flag = Some(flag);
                        break;
                    }
                }

                if let Some(flag) = matched_flag {
                    if let Some(val) = value_part {
                        // --key=value
                        if flag.takes_value {
                            flags_map.insert(flag.name.clone(), Some(val.to_string()));
                        } else {
                            return Err(KoralError::Validation(format!(
                                "Flag '--{}' does not take a value",
                                flag.name
                            )));
                        }
                    } else {
                        // --key (consume next if needed)
                        self.consume_flag_value(flag, &mut iter, &mut flags_map)?;
                    }
                } else {
                    // Unknown long flag
                    if self.strict {
                        return Err(KoralError::Validation(format!("Unknown flag '{}'", arg)));
                    }
                    positionals.push(arg.clone());
                }
            } else if arg.starts_with('-') && arg.len() > 1 {
                // Short flag (potentially combined)
                let char_part = arg.trim_start_matches('-');
                let chars: Vec<char> = char_part.chars().collect();

                // Check if all chars match known short flags to decide if we should treat this as a flag group
                // OR just try to match greedily.
                // Standard: if it starts with -, it's a flag group.
                // Exceptions: negative numbers? "-100" could be positional if no flags match?
                // For now, assume strict flag interpretation if it matches at least one?
                // Let's implement logic: iterate chars.

                // Let's implement logic: iterate chars.
                // Optimization/Heuristic: Check if first char is known?
                // Actually, if we encounter an unknown char inside grouping, what happens?
                // Error? Or treat entire string as positional?
                // "strict mode" isn't implemented yet, but for now let's say:
                // If the first char is NOT a flag, treat as positional (common for -1 etc).
                // If it IS a flag, treat as flag group, and error on unknown subsequent chars?

                // Let's use a simpler approach: Treat as flag group. Validation happens.
                // But we must support negative numbers if no flag matches.
                // Let's verify first char.
                let first_char = chars[0];
                let first_match = self.find_short_flag(first_char);

                if first_match.is_none() {
                    if self.strict {
                        return Err(KoralError::Validation(format!(
                            "Unknown short flag '{}' in '{}'",
                            first_char, arg
                        )));
                    }
                    // Treat as positional
                    positionals.push(arg.clone());
                    continue;
                }

                for (i, &c) in chars.iter().enumerate() {
                    let matched = self.find_short_flag(c);
                    if let Some(flag) = matched {
                        if flag.takes_value {
                            // If takes value
                            // 1. If not last char, rest of string is value.
                            if i < chars.len() - 1 {
                                let val = char_part.chars().skip(i + 1).collect::<String>();
                                flags_map.insert(flag.name.clone(), Some(val));
                                break; // Rest consumed
                            } else {
                                // 2. If last char, consume next arg
                                self.consume_flag_value(flag, &mut iter, &mut flags_map)?;
                            }
                        } else {
                            // Boolean
                            flags_map.insert(flag.name.clone(), None);
                        }
                    } else {
                        // Unknown flag in group
                        // What to do?
                        // If we are "strict", error.
                        // If "loose", maybe ignore?
                        // Returning error seems safest for now to avoid confusion.
                        return Err(KoralError::Validation(format!(
                            "Unknown short flag '-{}' in group '{}'",
                            c, arg
                        )));
                    }
                }
            } else {
                positionals.push(arg.clone());
            }
        }

        // Fill in default values if missing
        for flag in &self.known_flags {
            if !flags_map.contains_key(&flag.name) {
                // Check env
                let mut env_found = false;
                if let Some(env_var) = &flag.env {
                    if let Ok(val) = std::env::var(env_var) {
                        if flag.takes_value {
                            flags_map.insert(flag.name.clone(), Some(val));
                            env_found = true;
                        } else {
                            // For boolean: check if value looks like true (not 0 or false)
                            if val != "0" && val.to_lowercase() != "false" && !val.is_empty() {
                                flags_map.insert(flag.name.clone(), None);
                                env_found = true;
                            }
                        }
                    }
                }

                if !env_found {
                    if let Some(default) = &flag.default_value {
                        flags_map.insert(flag.name.clone(), Some(default.clone()));
                    }
                }
            }
        }

        // Validate flags
        for flag in &self.known_flags {
            if let Some(validator) = flag.validator {
                if let Some(Some(val)) = flags_map.get(&flag.name) {
                    if let Err(e) = validator(val) {
                        return Err(KoralError::Validation(format!(
                            "Invalid value for flag '{}': {}",
                            flag.name, e
                        )));
                    }
                }
            }
        }

        Ok(Context::new(flags_map, positionals))
    }

    fn find_short_flag(&self, c: char) -> Option<&crate::flag::FlagDef> {
        for flag in &self.known_flags {
            if let Some(s) = flag.short {
                if s == c {
                    return Some(flag);
                }
            }
        }
        None
    }

    fn consume_flag_value<'a, I>(
        &self,
        flag: &crate::flag::FlagDef,
        iter: &mut I,
        flags_map: &mut HashMap<String, Option<String>>,
    ) -> KoralResult<()>
    where
        I: Iterator<Item = &'a String>,
    {
        if flag.takes_value {
            if let Some(val) = iter.next() {
                flags_map.insert(flag.name.clone(), Some(val.clone()));
            } else {
                return Err(KoralError::MissingArgument(format!(
                    "Flag '--{}' requires a value",
                    flag.name
                )));
            }
        } else {
            // Boolean flag
            flags_map.insert(flag.name.clone(), None);
        }
        Ok(())
    }
}
