use koral::prelude::*;
#[derive(Debug)]
enum MyError {
    SomethingWrong,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Something went wrong")
    }
}

impl std::error::Error for MyError {}

struct MyApp;

impl AppTrait for MyApp {
    fn name(&self) -> &str {
        "test-app"
    }

    fn execute(&mut self, _ctx: Context) -> KoralResult<()> {
        // Test direct returning of custom error wrapped in KoralResult (clap::Error)
        Err(clap::Error::raw(
            clap::error::ErrorKind::Io,
            MyError::SomethingWrong,
        ))
    }
}

// Test using the extension trait
struct ExtApp;

impl AppTrait for ExtApp {
    fn name(&self) -> &str {
        "ext-app"
    }

    fn execute(&mut self, _ctx: Context) -> KoralResult<()> {
        // This function returns Result<(), MyError>
        fn fallible() -> Result<(), MyError> {
            Err(MyError::SomethingWrong)
        }

        // Use .koral_err()?
        fallible().koral_err()?;
        Ok(())
    }
}

#[test]
fn test_custom_error_wrapping() {
    let mut app = MyApp;
    let result = app.execute(Context::new(Default::default(), vec![]));
    match result {
        Err(e) if e.kind() == clap::error::ErrorKind::Io => {
            assert!(e.to_string().contains("Something went wrong"));
        }
        _ => panic!("Expected ErrorKind::Io, got {:?}", result),
    }
}

#[test]
fn test_extension_trait() {
    let mut app = ExtApp;
    let result = app.execute(Context::new(Default::default(), vec![]));
    match result {
        Err(e) if e.kind() == clap::error::ErrorKind::Io => {
            assert!(e.to_string().contains("Something went wrong"));
        }
        _ => panic!(
            "Expected ErrorKind::Io from extension trait, got {:?}",
            result
        ),
    }
}
