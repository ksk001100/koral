use koral::{App, Flag, KoralResult};

fn main() -> KoralResult<()> {
    let verbose_flag = Flag::<bool>::new("verbose")
        .alias("v")
        .description("Run with verbose output");
    
    let count_flag = Flag::<i32>::new("count")
        .default_value(1)
        .description("Number of times to say hello");

    App::new("simple-app")
        .version("1.0")
        .description("A simple app example")
        .flag(verbose_flag)
        .flag(count_flag)
        .action(|ctx| {
            let verbose = ctx.get::<bool>("verbose").unwrap_or(false);
            // Since we know default value is 1, but ctx.get returns Option matching parsing logic.
            // But Parser doesn't know about default definition in Flag struct currently!
            // Wait, Parser only sees &dyn Flag. 
            // My Parser implementation checks keys. 
            // It does NOT fill in default values if missing!
            // This is a missing piece in my Plan.
            // 
            // However, to fix this for now, we can use Option::unwrap_or here or rely on the fact 
            // that I need to update Parser or Context to handle defaults?
            // Actually, Flag struct has `default_value`. 
            // But Context doesn't know about it.
            // The user must handle it or we update Parser to fill defaults.
            // 
            // Plan said: "Remove parse method... Logic moves to Parser".
            // Old `Flag::parse` returned `default_value` if not found.
            // My new Parser returns Context with what is present.
            // So `ctx.get()` will return None if not present.
            // The user code needs `unwrap_or(1)`.
            // OR I should have updated Parser to start with defaults.
            // 
            // Let's check Parser implementation. 
            // It iterates known_flags. I can fill defaults there!
            // BUT `dyn Flag` trait doesn't expose `default_value`! `Flag` struct has it.
            // `Flag` trait only has `name`, `description`, `aliases` + `takes_value`.
            // `Flag` struct is generic `Flag<T>`, so we can't expose `T` in `dyn Flag`.
            // 
            // So `ctx.get` cannot return defaults easily from `dyn Flag`.
            // The user has to provide defaults in code like `unwrap_or(default)`.
            // OR we change design to `Flag::new(...).default(1)`.
            // But `App` stores `Box<dyn Flag>`.
            // 
            // For this iteration, I will manually handle defaults in the example.
            
            let count = ctx.get::<i32>("count").unwrap_or(1);

            if verbose {
                println!("Verbose mode on");
            }

            for i in 0..count {
                println!("Hello #{}", i + 1);
            }
            
            if !ctx.args.is_empty() {
                println!("Positional arguments: {:?}", ctx.args);
            }
            Ok(())
        })
        .run(std::env::args().skip(1).collect())
}
