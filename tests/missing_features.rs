use koral::prelude::*;
use koral::App as AppBuilder;
use std::sync::{Arc, Mutex};

#[derive(Flag, Clone)]
#[flag(name = "flag", short = 'f')]
struct FlagF(#[allow(dead_code)] String);

#[test]
fn test_delimiter() {
    let captured_args = Arc::new(Mutex::new(Vec::new()));
    let captured_args_clone = captured_args.clone();

    // Use App builder
    let mut app = AppBuilder::new("test-app")
        .register::<FlagF>()
        .action(move |ctx| {
            let mut args = captured_args_clone.lock().unwrap();
            *args = ctx.args.to_vec();
            Ok(())
        });

    // Run with delimiter
    let args = vec!["prog", "--", "-f", "value"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    app.run(args).unwrap();

    let args = captured_args.lock().unwrap();
    assert_eq!(*args, vec!["-f", "value"]);
}

#[test]
fn test_negative_number() {
    let captured_args = Arc::new(Mutex::new(Vec::new()));
    let captured_args_clone = captured_args.clone();

    // Strict mode to ensure it handles negatives correctly even in strict
    let mut app = AppBuilder::new("test-app")
        .strict(true)
        .register::<FlagF>()
        .action(move |ctx| {
            let mut args = captured_args_clone.lock().unwrap();
            *args = ctx.args.to_vec();
            Ok(())
        });

    // Run with negative number
    let args = vec!["prog", "-1", "-3.14"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    app.run(args).unwrap();

    let args = captured_args.lock().unwrap();
    // Assuming strict mode allows negative numbers if they look like numbers
    assert_eq!(*args, vec!["-1", "-3.14"]);
}

#[test]
fn test_typo_correction() {
    let mut app = AppBuilder::new("test-app").strict(true).register::<FlagF>();

    let args = vec!["prog", "--flagg"] // typo for --flag
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    let err = app.run(args).unwrap_err();
    match err {
        KoralError::UnknownFlag(msg) => {
            assert!(msg.contains("--flag"));
        }
        _ => panic!("Expected UnknownFlag error, got {:?}", err),
    }
}

#[test]
fn test_version_flag_parsed() {
    // We can't easily capture stdout, but we can verify that run() returns Ok
    // and that it didn't call the action (if version handles it).

    let action_called = Arc::new(Mutex::new(false));
    let action_called_clone = action_called.clone();

    let mut app = AppBuilder::new("test-app")
        .version("1.2.3")
        .action(move |_| {
            *action_called_clone.lock().unwrap() = true;
            Ok(())
        });

    let args = vec!["prog", "--version"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    app.run(args).unwrap();

    // Action should NOT be called because version flag intercepts it
    assert!(!*action_called.lock().unwrap());
}

#[test]
fn test_man_page_generation() {
    let app = AppBuilder::new("test-app")
        .description("Test Description")
        .register::<FlagF>();

    let man = koral::man::generate_man_page(&app, "Jan 2026");

    assert!(man.contains(".TH \"TEST-APP\" 1 \"Jan 2026\""));
    assert!(man.contains(".SH NAME"));
    assert!(man.contains("test-app \\- Test Description"));
    assert!(man.contains(".SH OPTIONS"));
    assert!(man.contains("\\fB--flag\\fR"));
}
