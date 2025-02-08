use koral::App;

fn main() {
    let app = App::new("app")
        .action(|ctx| {
            println!("app");
            println!("args: {:?}", ctx);
            Ok(())
        })
        .app(
            App::new("nest_app1")
                .action(|ctx| {
                    println!("nest_app1");
                    println!("args: {:?}", ctx);
                    Ok(())
                })
                .app(App::new("nest_app2").action(|ctx| {
                    println!("nest_app2");
                    println!("args: {:?}", ctx);
                    Ok(())
                })),
        );

    match app.run(std::env::args().collect()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
