use koral::prelude::*;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

// Custom Enum
#[derive(Debug, Clone, PartialEq, FlagValue)]
enum Color {
    Red,
    Green,
    Blue,
}

// Custom Struct
#[derive(Debug, Clone, PartialEq)]
struct Speed(u32);

impl FromStr for Speed {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val: u32 = s.parse().map_err(|_| "Invalid number".to_string())?;
        if val > 100 {
            return Err("Too fast".to_string());
        }
        Ok(Speed(val))
    }
}

impl ToString for Speed {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

// Flags
#[derive(Flag, Debug, Clone, PartialEq)]
#[flag(name = "color", short = 'c')]
struct ColorFlag(Color);

#[derive(Flag, Debug, Clone, PartialEq)]
#[flag(name = "speed", short = 's')]
struct SpeedFlag(Speed);

#[derive(App, Default)]
#[app(name = "custom_test", action = test_action)]
#[app(flags(ColorFlag, SpeedFlag))]
struct TestApp;

#[derive(Clone, Debug, Default)]
struct TestResult {
    color: Option<String>,
    speed: Option<u32>,
}

fn test_action(ctx: Context) -> KoralResult<()> {
    let res = TestResult {
        color: ctx.get::<ColorFlag>().map(|c| format!("{:?}", c)),
        speed: ctx.get::<SpeedFlag>().map(|s| s.0),
    };

    let state = ctx
        .state::<Arc<Mutex<Option<TestResult>>>>()
        .expect("State missing");
    let mut guard = state.lock().unwrap();
    *guard = Some(res);
    Ok(())
}

fn run_test(args: Vec<&str>) -> KoralResult<TestResult> {
    let result_store = Arc::new(Mutex::new(None));
    let mut app = TestApp;
    let mut state: Arc<Mutex<Option<TestResult>>> = result_store.clone();

    let args: Vec<String> = args.into_iter().map(|s| s.to_string()).collect();
    app.run_with_state(&mut state, args)?;

    let guard = result_store.lock().unwrap();
    guard
        .clone()
        .ok_or(KoralError::Validation("Action did not run".to_string()))
}

#[test]
fn test_enum_parsing() {
    let res = run_test(vec!["prog", "--color", "Red"]).unwrap();
    assert_eq!(res.color, Some("Red".to_string()));

    let res = run_test(vec!["prog", "--color", "Green"]).unwrap();
    assert_eq!(res.color, Some("Green".to_string()));
}

#[test]
fn test_enum_error() {
    let res = run_test(vec!["prog", "--color", "Yellow"]).unwrap();
    assert_eq!(res.color, None, "Invalid enum should result in None");
}

#[test]
fn test_struct_parsing() {
    let res = run_test(vec!["prog", "--speed", "50"]).unwrap();
    assert_eq!(res.speed, Some(50));
}

#[test]
fn test_struct_error() {
    let res = run_test(vec!["prog", "--speed", "150"]).unwrap(); // Too fast
    assert_eq!(
        res.speed, None,
        "Invalid struct range should result in None"
    );

    let res = run_test(vec!["prog", "--speed", "abc"]).unwrap(); // Invalid number
    assert_eq!(
        res.speed, None,
        "Invalid struct format should result in None"
    );
}
