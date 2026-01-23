use koral::completion::{generate_to, Shell};
use koral::prelude::*;

#[derive(Flag, Debug, Clone, PartialEq)]
#[flag(name = "verbose", short = 'v')]
struct VerboseFlag;

#[derive(Flag, Debug, Clone, PartialEq)]
#[flag(name = "user", short = 'u')]
struct UserFlag(String);

#[derive(App, Default)]
#[app(name = "myprog", action = run)]
#[app(flags(VerboseFlag, UserFlag))]
struct MyApp;

fn run(_ctx: Context) -> KoralResult<()> {
    Ok(())
}

#[test]
fn test_bash_completion() {
    let mut buf = Vec::new();
    // generate_to takes &App, Shell, &mut Write
    // MyApp implements App.
    // Wait, generate_to might need the actual object?
    // "fn generate_to<A: App + ?Sized>(app: &A, shell: Shell, buf: &mut dyn Write) -> std::io::Result<()>"

    let app = MyApp;
    generate_to(&app, Shell::Bash, &mut buf).unwrap();

    let output = String::from_utf8(buf).unwrap();
    assert!(output.contains("_myprog")); // clap_complete uses _myprog
    assert!(output.contains("--verbose"));
    assert!(output.contains("--user"));
    assert!(output.contains("-v"));
    assert!(output.contains("-u"));
}

#[test]
fn test_zsh_completion() {
    let mut buf = Vec::new();
    let app = MyApp;
    generate_to(&app, Shell::Zsh, &mut buf).unwrap();

    let output = String::from_utf8(buf).unwrap();
    assert!(output.contains("#compdef myprog"));
    assert!(output.contains("--verbose"));
    assert!(output.contains("--user"));
}

#[test]
fn test_fish_completion() {
    let mut buf = Vec::new();
    let app = MyApp;
    generate_to(&app, Shell::Fish, &mut buf).unwrap();

    let output = String::from_utf8(buf).unwrap();
    std::fs::write("completion_debug.txt", &output).unwrap();

    assert!(output.contains("complete -c myprog"));

    // Loose assertions
    assert!(output.contains("-s v"));
    assert!(output.contains("-l verbose"));
}
