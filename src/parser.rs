use crate::context::Context;
use crate::error::{KoralError, KoralResult};
use std::collections::HashMap;

pub struct Parser {
    known_flags: Vec<crate::flag::FlagDef>,
}

impl Parser {
    pub fn new(flags: Vec<crate::flag::FlagDef>) -> Self {
        Self { known_flags: flags }
    }

    pub fn parse<'a>(&self, args: &[String]) -> KoralResult<Context<'a>> {
        let mut flags_map: HashMap<String, Option<String>> = HashMap::new();
        let mut positionals: Vec<String> = Vec::new();

        let mut iter = args.iter();
        while let Some(arg) = iter.next() {
            if arg.starts_with("--") {
                // Long flag
                let name_in_arg = arg.trim_start_matches("--");

                // Find matching flag
                let mut matched_flag: Option<&crate::flag::FlagDef> = None;
                for flag in &self.known_flags {
                    let flag_long = flag.long.as_deref().unwrap_or(&flag.name);
                    if flag_long == name_in_arg {
                        matched_flag = Some(flag);
                        break;
                    }
                }

                if let Some(flag) = matched_flag {
                    self.consume_flag_value(flag, &mut iter, &mut flags_map)?;
                } else {
                    // Unknown long flag
                    positionals.push(arg.clone());
                }
            } else if arg.starts_with('-') && arg.len() > 1 {
                // Short flag (potentially)
                let char_part = arg.trim_start_matches('-');
                if char_part.len() == 1 {
                    let c = char_part.chars().next().unwrap();
                    let mut matched_flag: Option<&crate::flag::FlagDef> = None;
                    for flag in &self.known_flags {
                        if let Some(s) = flag.short {
                            if s == c {
                                matched_flag = Some(flag);
                                break;
                            }
                        }
                    }

                    if let Some(flag) = matched_flag {
                        self.consume_flag_value(flag, &mut iter, &mut flags_map)?;
                    } else {
                        positionals.push(arg.clone());
                    }
                } else {
                    // Unknown or combined short flags
                    positionals.push(arg.clone());
                }
            } else {
                positionals.push(arg.clone());
            }
        }

        // Fill in default values if missing
        for flag in &self.known_flags {
            if !flags_map.contains_key(&flag.name) {
                if let Some(default) = &flag.default_value {
                    flags_map.insert(flag.name.clone(), Some(default.clone()));
                }
            }
        }

        Ok(Context::new(flags_map, positionals))
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
