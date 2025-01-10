use koral::Koral;

fn main() {
    let koral = Koral::new("hello").action(|_| {
        println!("Hello, world!");
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
