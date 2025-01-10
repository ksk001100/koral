use koral::*;

fn main() {
    let koral = Koral::new("app")
        .action(|_| {
            println!("app");
            Ok(())
        })
        .app(
            Koral::new("nest_app1")
                .action(|_| {
                    println!("nest_app1");
                    Ok(())
                })
                .app(Koral::new("nest_app2").action(|_| {
                    println!("nest_app2");
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
