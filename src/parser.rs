use crate::error::{KoralError, KoralResult};
use crate::traits::Flag;
use crate::context::Context;
use std::collections::HashMap;

pub struct Parser<'a> {
    known_flags: Vec<&'a dyn Flag>,
}

impl<'a> Parser<'a> {
    pub fn new(flags: Vec<&'a dyn Flag>) -> Self {
        Self { known_flags: flags }
    }

    pub fn parse(&mut self, args: &[String]) -> KoralResult<Context> {
        let mut flags_map: HashMap<String, Option<String>> = HashMap::new();
        let mut positionals: Vec<String> = Vec::new();

        let mut iter = args.iter();
        while let Some(arg) = iter.next() {
            if arg.starts_with('-') { // It's a flag
                 // Remove dashes to find name
                 let trimmed = arg.trim_start_matches('-');
                 
                 // Find matching flag
                 // Check name or aliases
                 let mut matched_flag: Option<&dyn Flag> = None;
                 
                 for flag in &self.known_flags {
                     if flag.name() == trimmed || flag.aliases().contains(&trimmed) {
                         matched_flag = Some(*flag);
                         break;
                     }
                 }

                 if let Some(flag) = matched_flag {
                     let name = flag.name().to_string();
                     if flag.takes_value() {
                         // Expect next arg
                         if let Some(val) = iter.next() {
                             // Check if val looks like a flag?
                             // Typically if a flag requires a value, we consume strict next.
                             // But some CLIs allow skipped optional values, but Koral design says "requires value".
                             flags_map.insert(name, Some(val.clone()));
                         } else {
                             return Err(KoralError::MissingArgument(format!("Flag '{}' requires a value", name)));
                         }
                     } else {
                         // Boolean flag, no value needed.
                         // We store None to indicate presence.
                         // But Context getter handles boolean specially too.
                         flags_map.insert(name, None);
                     }
                 } else {
                     // Unknown flag behavior
                     // Treat as positional argument to allow subcommand flags to pass through
                     positionals.push(arg.clone());
                 }
            } else {
                positionals.push(arg.clone());
            }
        }
        
        Ok(Context::new(flags_map, positionals))
    }
}

