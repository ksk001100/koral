use koral::prelude::*;
use std::str::FromStr;

// Multi-field struct
#[derive(Clone, Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

// Manual implementation of FromStr
// Format: "name,age" (e.g. "Alice,30")
impl FromStr for Person {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err("Format must be 'name,age'".to_string());
        }

        let name = parts[0].to_string();
        let age = parts[1].parse::<u32>().map_err(|e| e.to_string())?;

        Ok(Person { name, age })
    }
}

impl ToString for Person {
    fn to_string(&self) -> String {
        format!("{},{}", self.name, self.age)
    }
}

// Flag Definition
// Note: We don't use derive(FlagValue) here because it doesn't support multi-field structs.
// But since we implemented FromStr + ToString + Clone + Send + Sync manually,
// Person automatically implements FlagValue trait!
fn validate_person(s: &str) -> Result<(), String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err("Format must be 'name,age'".to_string());
    }
    let age = parts[1]
        .parse::<u32>()
        .map_err(|_| "Age must be a number".to_string())?;
    if age == 0 {
        return Err("Age must be positive".to_string());
    }
    Ok(())
}

#[derive(Flag, Debug)]
#[flag(
    name = "person",
    short = 'p',
    help = "Person info in 'name,age' format",
    validator = validate_person,
    aliases = "user, human"
)]
struct PersonFlag(#[allow(dead_code)] Person);

#[derive(App)]
#[app(name = "complex_flag", action = run)]
#[app(flags(PersonFlag))]
struct App;

fn run(ctx: Context<App>) -> KoralResult<()> {
    if let Some(person) = ctx.get::<PersonFlag>() {
        println!("Received Person: {:?}", person);
    } else {
        println!("No person provided");
    }
    Ok(())
}

fn main() -> KoralResult<()> {
    App.run(std::env::args().collect())
}
