use koral::Koral;

fn main() {
    let koral = Koral::new("app")
        .action(|ctx| {
            println!("app");
            println!("args: {:?}", ctx);
            Ok(())
        })
        .app(
            Koral::new("nest_app1")
                .action(|ctx| {
                    println!("nest_app1");
                    println!("args: {:?}", ctx);
                    Ok(())
                })
                .app(Koral::new("nest_app2").action(|ctx| {
                    println!("nest_app2");
                    println!("args: {:?}", ctx);
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
