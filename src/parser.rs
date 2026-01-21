use crate::context::Context;
use crate::error::{KoralError, KoralResult};
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
        let mut flags_map: HashMap<String, Option<String>> = HashMap::new();

        let mut positionals: Vec<String> = Vec::new();

        let mut iter = args.iter();
        while let Some(arg) = iter.next() {
            if arg == "--" {
                // consume all remaining arguments as positionals
                for remaining in iter {
                    positionals.push(remaining.clone());
                }
                break;
            }

            if arg.starts_with("--") {
                // Long flag
                self.parse_long_flag(arg, &mut iter, &mut flags_map, &mut positionals)?;
            } else if arg.starts_with('-') && arg.len() > 1 {
                // Short flag (potentially combined)
                self.parse_short_flags(arg, &mut iter, &mut flags_map, &mut positionals)?;
            } else {
                positionals.push(arg.clone());
            }
        }

        // Fill in default values if missing
        self.apply_defaults(&mut flags_map);

        // Validate flags
        self.validate_constraints(&flags_map)?;

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

    fn parse_long_flag<'a, I>(
        &self,
        arg: &str,
        iter: &mut I,
        flags_map: &mut HashMap<String, Option<String>>,
        positionals: &mut Vec<String>,
    ) -> KoralResult<()>
    where
        I: Iterator<Item = &'a String>,
    {
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
                self.consume_flag_value(flag, iter, flags_map)?;
            }
        } else {
            // Unknown long flag
            if self.strict {
                // Check for typos
                let mut msg = format!("Unknown flag '{}'", arg);
                if let Some(sugg) = self.suggest_flag(name_part) {
                    msg.push_str(&format!("\n\tDid you mean '{}'?", sugg));
                }
                return Err(KoralError::UnknownFlag(msg));
            }
            positionals.push(arg.to_string());
        }
        Ok(())
    }

    fn parse_short_flags<'a, I>(
        &self,
        arg: &str,
        iter: &mut I,
        flags_map: &mut HashMap<String, Option<String>>,
        positionals: &mut Vec<String>,
    ) -> KoralResult<()>
    where
        I: Iterator<Item = &'a String>,
    {
        let char_part = arg.trim_start_matches('-');
        let chars: Vec<char> = char_part.chars().collect();

        // Check first char logic/heuristics
        let first_char = chars[0];
        let first_match = self.find_short_flag(first_char);

        if first_match.is_none() {
            // Check if it looks like a negative number
            let is_number = char_part.parse::<f64>().is_ok();

            if is_number {
                positionals.push(arg.to_string());
                return Ok(());
            }

            if self.strict {
                return Err(KoralError::UnknownFlag(format!(
                    "Unknown short flag '{}' in '{}'",
                    first_char, arg
                )));
            }
            // Treat as positional
            positionals.push(arg.to_string());
            return Ok(());
        }

        // Atomic parsing for flag group
        struct PlanItem<'p> {
            flag: &'p crate::flag::FlagDef,
            val: Option<String>,
        }
        let mut plan: Vec<PlanItem> = Vec::new();
        let mut valid_group = true;
        let mut consume_next: Option<&crate::flag::FlagDef> = None;

        for (i, &c) in chars.iter().enumerate() {
            if let Some(flag) = self.find_short_flag(c) {
                if flag.takes_value {
                    if i < chars.len() - 1 {
                        // Value attached
                        let val = char_part.chars().skip(i + 1).collect::<String>();
                        plan.push(PlanItem {
                            flag,
                            val: Some(val),
                        });
                        break; // Rest consumed
                    } else {
                        // Value is next arg
                        consume_next = Some(flag);
                        break;
                    }
                } else {
                    // Boolean
                    plan.push(PlanItem { flag, val: None });
                }
            } else {
                // Unknown flag
                valid_group = false;
                if self.strict {
                    return Err(KoralError::UnknownFlag(format!(
                        "Unknown short flag '-{}' in group '{}'",
                        c, arg
                    )));
                }
                break;
            }
        }

        if valid_group {
            // Apply plan
            for item in plan {
                if let Some(val) = item.val {
                    flags_map.insert(item.flag.name.clone(), Some(val));
                } else {
                    flags_map.insert(item.flag.name.clone(), None);
                }
            }
            if let Some(flag) = consume_next {
                self.consume_flag_value(flag, iter, flags_map)?;
            }
        } else {
            // Treated as positional in non-strict mode
            positionals.push(arg.to_string());
        }
        Ok(())
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
                            // For boolean flags from env/defaults
                            // If val is "true" or non-empty/non-zero
                            if val != "0" && val.to_lowercase() != "false" && !val.is_empty() {
                                flags_map.insert(flag.name.clone(), None);
                            }
                        }
                        // Once found, stop checking other providers
                        break;
                    }
                }
            }
        }
    }

    fn validate_constraints(&self, flags_map: &HashMap<String, Option<String>>) -> KoralResult<()> {
        for flag in &self.known_flags {
            // Check required
            if !self.ignore_required && flag.required && !flags_map.contains_key(&flag.name) {
                return Err(KoralError::MissingArgument(format!(
                    "Required flag '--{}' is missing",
                    flag.name
                )));
            }

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
        Ok(())
    }

    fn suggest_flag(&self, unknown: &str) -> Option<String> {
        let mut best_match: Option<String> = None;
        let mut min_dist = usize::MAX;

        for flag in &self.known_flags {
            let name = &flag.name;
            let dist = levenshtein(unknown, name);
            if dist < min_dist && dist <= 3 {
                // Threshold
                min_dist = dist;
                best_match = Some(format!("--{}", name));
            }
            if let Some(short) = flag.short {
                let s = short.to_string();
                let dist = levenshtein(unknown, &s);
                // Short flag usually not typo target unless it's very short input
                if dist < min_dist && dist <= 1 {
                    min_dist = dist;
                    best_match = Some(format!("-{}", s));
                }
            }
        }
        best_match
    }
}

fn levenshtein(a: &str, b: &str) -> usize {
    let len_a = a.chars().count();
    let len_b = b.chars().count();
    if len_a == 0 {
        return len_b;
    }
    if len_b == 0 {
        return len_a;
    }

    let mut matrix = vec![vec![0; len_b + 1]; len_a + 1];

    for i in 0..=len_a {
        matrix[i][0] = i;
    }
    for j in 0..=len_b {
        matrix[0][j] = j;
    }

    for (i, ca) in a.chars().enumerate() {
        for (j, cb) in b.chars().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            matrix[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(matrix[i][j + 1] + 1, matrix[i + 1][j] + 1),
                matrix[i][j] + cost,
            );
        }
    }
    matrix[len_a][len_b]
}

/// Helper function to validate required flags externally.
/// Used by generated App code to enforce requirements only when specific action is executed.
pub fn validate_required_flags(
    flags: &[crate::flag::FlagDef],
    flags_map: &HashMap<String, Option<String>>,
) -> KoralResult<()> {
    for flag in flags {
        if flag.required && !flags_map.contains_key(&flag.name) {
            return Err(KoralError::MissingArgument(format!(
                "Required flag '--{}' is missing",
                flag.name
            )));
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
        assert!(matches!(err, Err(KoralError::MissingArgument(_))));

        // Case 3: Required present, Optional missing (OK)
        let mut map3 = HashMap::new();
        map3.insert("req".to_string(), Some("val".to_string()));
        assert!(validate_required_flags(&flags, &map3).is_ok());
    }
}
