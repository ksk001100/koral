use koral::prelude::*;
use koral::App as AppBuilder;

#[derive(Flag, Clone)]
#[flag(name = "flag", short = 'f')]
struct FlagF(#[allow(dead_code)] bool);

#[test]
fn test_strict_mode_group_error() {
    let mut app = AppBuilder::new("test").strict(true).register::<FlagF>();

    // -f is known, x is unknown
    let args = vec!["prog", "-fx"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let res = app.run(args);
    assert!(res.is_err());
    let err = res.unwrap_err();
    match err {
        KoralError::UnknownFlag(msg) => {
            println!("Strict Error: {}", msg);
            assert!(msg.contains("Unknown short flag '-x'"));
        }
        _ => panic!("Expected UnknownFlag error, got {:?}", err),
    }
}

#[test]
fn test_non_strict_mode_group_behavior() {
    use std::sync::{Arc, Mutex};
    let captured_args = Arc::new(Mutex::new(Vec::new()));
    let captured_clone = captured_args.clone();

    let mut app = AppBuilder::new("test")
        .strict(false) // Non-strict
        .register::<FlagF>()
        .action(move |ctx| {
            let mut args = captured_clone.lock().unwrap();
            *args = ctx.args.to_vec();
            Ok(())
        });

    // -f is known, x is unknown.
    // In strict mode: Error.
    // In non-strict mode: Treat "-fx" as positional.
    // So -f is NOT applied.
    let args = vec!["prog", "-fx"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    app.run(args).expect("Non-strict mode should not error");

    let args = captured_args.lock().unwrap();
    assert_eq!(*args, vec!["-fx"]);
}
