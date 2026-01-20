use crate::TodoApp;
use koral::prelude::*;

#[derive(App, Default)]
#[app(name = "completion", action = completion_action)]
pub struct CompletionCmd;

fn completion_action(ctx: Context<CompletionCmd>) -> KoralResult<()> {
    let shell_arg = ctx.args.first().map(|s| s.as_str()).unwrap_or("bash");
    let shell = match shell_arg {
        "bash" => koral::Shell::Bash,
        "zsh" => koral::Shell::Zsh,
        "fish" => koral::Shell::Fish,
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Unknown shell: {}", shell_arg),
            )
            .into())
        }
    };

    let app = TodoApp::default();
    koral::generate_to(&app, shell, &mut std::io::stdout()).map_err(|e| e.into())
}
