use koral::prelude::*;
use std::sync::{Arc, Mutex};

// Define a flag with short 'h'
struct HeaderFlag;
impl Flag for HeaderFlag {
    type Value = String;
    fn name() -> &'static str {
        "header"
    }
    fn short() -> Option<char> {
        Some('h')
    }
    fn takes_value() -> bool {
        true
    }
}

#[test]
fn test_h_flag_override() {
    let executed = Arc::new(Mutex::new(false));
    let executed_clone = executed.clone();

    let mut app = App::new("test")
        .register::<HeaderFlag>()
        .action(move |ctx| {
            let mut guard = executed_clone.lock().unwrap();
            *guard = true;
            // Expect that -h was parsed as header flag
            assert_eq!(ctx.get::<HeaderFlag>(), Some("foo".to_string()));
            Ok(())
        });

    // Arguments: "test -h foo"
    // Expectation: If -h works as header flag, action is executed.
    // Issue: -h triggers help, action is NOT executed.
    let args = vec!["test".to_string(), "-h".to_string(), "foo".to_string()];
    let res = app.run(args);

    // The run command returns Ok(()) if help is printed, or result of action.
    assert!(res.is_ok());

    assert!(
        *executed.lock().unwrap(),
        "-h should be treated as header flag, not help"
    );
}
