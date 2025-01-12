use koral::{Flag, FlagKind, Koral};

fn main() {
    let koral = Koral::new("hello")
        .flag(Flag::new("test", FlagKind::Boolean))
        .flag(Flag::new("name", FlagKind::Value))
        .action(|c| {
            println!("Hello, world!");
            println!("Flags: {:?}", c.flags);
            println!("Args: {:?}", c.args);
            Ok(())
        });

    match koral.run(std::env::args().collect()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
