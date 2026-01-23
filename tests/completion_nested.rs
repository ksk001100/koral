use koral::completion::{generate_to, Shell};
use koral::prelude::*;

#[derive(Default, App)]
#[app(name = "root", version = "1.0", action = run_root)]
#[app(subcommands(RootCmds))]
struct RootApp;

fn run_root(_: Context) -> KoralResult<()> {
    Ok(())
}

#[derive(Subcommand)]
enum RootCmds {
    #[subcommand(name = "sub", description = "Sub command")]
    Sub(SubApp),
}

#[derive(Default, App)]
#[app(name = "sub_app_ignored")] // This name is ignored when wrapped in Enum usually, but fields are used
#[app(subcommands(SubCmds))]
#[app(flags(SubFlag))]
struct SubApp;

#[derive(Flag, Clone)]
#[flag(name = "subflag", help = "Flag in subcommand")]
struct SubFlag(#[allow(dead_code)] bool);

#[derive(Subcommand)]
enum SubCmds {
    #[subcommand(name = "deep", description = "Deep command")]
    Deep(DeepApp),
}

#[derive(Default, App)]
#[app(flags(DeepFlag))]
struct DeepApp;

#[derive(Flag, Clone)]
#[flag(name = "deepflag", help = "Flag in deep command")]
struct DeepFlag(#[allow(dead_code)] bool);

#[test]
fn test_nested_completion_generated() {
    let app = RootApp;
    let mut buf = Vec::new();
    generate_to(&app, Shell::Bash, &mut buf).unwrap();
    let output = String::from_utf8(buf).unwrap();

    // println!("{}", output); // Debug if needed

    // Verify root is present
    assert!(output.contains("_root"));

    // Verify immediate child 'sub'
    assert!(output.contains("root,sub"));

    // Verify nested 'deep'
    assert!(
        output.contains("root__sub,deep"),
        "Bash completion missing nested 'deep' case"
    );

    // Verify flags
    assert!(output.contains("--subflag"), "Missing --subflag");
    assert!(output.contains("--deepflag"), "Missing --deepflag");

    // Check Zsh
    let mut buf_zsh = Vec::new();
    generate_to(&app, Shell::Zsh, &mut buf_zsh).unwrap();
    let output_zsh = String::from_utf8(buf_zsh).unwrap();

    // Zsh completion from clap_complete
    assert!(output_zsh.contains("sub"), "Zsh missing 'sub'");
    assert!(output_zsh.contains("deep"), "Zsh missing 'deep'");

    // Check Fish
    let mut buf_fish = Vec::new();
    generate_to(&app, Shell::Fish, &mut buf_fish).unwrap();
    let output_fish = String::from_utf8(buf_fish).unwrap();

    // Fish completion from clap_complete
    assert!(output_fish.contains("complete -c root"));
    assert!(output_fish.contains("-a \"sub\"") || output_fish.contains("-a sub"));
    assert!(output_fish.contains("-a \"deep\"") || output_fish.contains("-a deep"));
    assert!(output_fish.contains("-l subflag"));
}
