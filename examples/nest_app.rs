use koral::Koral;

fn main() {
    let koral = Koral::new("app")
        .action(|args| {
            println!("app");
            println!("args: {:?}", args);
            Ok(())
        })
        .app(
            Koral::new("nest_app1")
                .action(|args| {
                    println!("nest_app1");
                    println!("args: {:?}", args);
                    Ok(())
                })
                .app(Koral::new("nest_app2").action(|args| {
                    println!("nest_app2");
                    println!("args: {:?}", args);
                    Ok(())
                })),
        );

    match koral.run(std::env::args().collect()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
