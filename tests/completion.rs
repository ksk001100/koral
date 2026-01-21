use koral::completion::{generate_to, Shell};
use koral::prelude::*;
use koral::App as AppBuilder;

#[derive(Flag, Clone)]
#[flag(name = "file", short = 'f', value_name = "FILE")]
struct FileFlag(#[allow(dead_code)] String);

#[test]
fn test_zsh_completion_hints() {
    let app = AppBuilder::new("test").register::<FileFlag>();

    let mut buf = Vec::new();
    generate_to(&app, Shell::Zsh, &mut buf).unwrap();
    let output = String::from_utf8(buf).unwrap();

    // Check for _files usage
    // Expected: '--file[...]:FILE:_files'
    // The exact format depends on help text, but :FILE:_files part should be there.
    assert!(output.contains(":FILE:_files"));
}
