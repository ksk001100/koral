use crate::parser::build_command;
use crate::traits::App;

/// Generate help message for the application using clap's built-in renderer.
pub fn generate_help<T: App + ?Sized>(app: &T) -> String {
    let mut cmd = build_command(app);
    cmd.render_help().to_string()
}
