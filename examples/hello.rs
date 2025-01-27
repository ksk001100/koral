use koral::{Flag, FlagKind, FlagTrait, Koral};

fn main() {
    let koral = Koral::new("hello")
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

    match koral.run(std::env::args().collect()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
