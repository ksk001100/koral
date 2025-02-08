use koral::{
    app::App,
    flag::{Flag, FlagKind},
};

fn main() {
    let app = App::new("hello")
        .flag(Flag::new("test", FlagKind::Boolean))
        .flag(Flag::new("name", FlagKind::Value))
        .flag(Flag::new("error", FlagKind::Boolean))
        .action(|c| {
            println!("Flags: {:?}", c.flags);
            println!("Args: {:?}", c.args);

            match c.bool_flag("error") {
                true => Err("An error occurred".into()),
                false => {
                    println!("Hello, world!");
                    Ok(())
                }
            }
        });

    match app.run(std::env::args().collect()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
