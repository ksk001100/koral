use crate::parser::build_command;
use crate::traits::App;
use clap_mangen::Man;

/// Generate ROFF man page for the application using clap_mangen.
/// `date` is currently ignored as clap_mangen usually handles this via metadata or environment.
pub fn generate_man_page<T: App + ?Sized>(app: &T, _date: &str) -> String {
    let cmd = build_command(app);
    let man = Man::new(cmd);
    let mut buffer = Vec::new();
    man.render(&mut buffer).expect("Failed to render man page");
    String::from_utf8(buffer).expect("Invalid UTF-8 in man page")
}
