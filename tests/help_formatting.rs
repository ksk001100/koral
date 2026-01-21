use koral::help::generate_help;
use koral::App as AppBuilder; // App is exposed as AppBuilder alias in integration tests convention usually

#[test]
fn test_help_wrapping() {
    unsafe {
        std::env::set_var("COLUMNS", "40");
    }

    let app = AppBuilder::new("test")
        .description("This is a very long description that should wrap across lines.");

    let help = generate_help(&app);
    println!("Help output:\n{}", help);

    // "This is a very long" (19)
    // "description that" matches wrapped?
    // "This is a very long description that should wrap across lines."
    // Wrapped at 40 width.
    // Indent is 0.
    // "This is a very long description that" (36 chars) fits?
    // "should wrap across lines." (25 chars)

    // Check if it wrapped
    assert!(!help.contains("This is a very long description that should wrap across lines."));
    assert!(help.contains("long"));
}
