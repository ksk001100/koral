use koral::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
enum MyError {
    #[error("Something went wrong")]
    SomethingWrong,
}

struct MyApp;

impl AppTrait for MyApp {
    fn name(&self) -> &str {
        "test-app"
    }

    fn execute(&mut self, _ctx: Context) -> KoralResult<()> {
        // Test direct returning of custom error wrapped in KoralError
        // This simulates a manual mapping if needed, or verifies `?` works if we had a helper that returned KoralResult
        Err(KoralError::Other(Box::new(MyError::SomethingWrong)))
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
        Err(KoralError::Other(e)) => {
            assert_eq!(e.to_string(), "Something went wrong");
            // Downcast check
            if let Some(my_err) = e.downcast_ref::<MyError>() {
                match my_err {
                    MyError::SomethingWrong => (), // OK
                }
            } else {
                panic!("Could not downcast to MyError");
            }
        }
        _ => panic!("Expected KoralError::Other"),
    }
}

#[test]
fn test_extension_trait() {
    let mut app = ExtApp;
    let result = app.execute(Context::new(Default::default(), vec![]));
    match result {
        Err(KoralError::Other(e)) => {
            assert_eq!(e.to_string(), "Something went wrong");
        }
        _ => panic!("Expected KoralError::Other from extension trait"),
    }
}
