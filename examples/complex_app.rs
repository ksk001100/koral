use koral::{Flag, KoralResult, Context};
use koral::traits::{App, Flag as FlagTrait};

// --- Sub-command: git remote add <name> <url> ---
struct RemoteAdd {
    name: String,
    url: String,
}

impl App for RemoteAdd {
    fn name(&self) -> &str { "add" }
    
    fn execute(&mut self, ctx: Context) -> KoralResult<()> {
        if ctx.args.len() < 2 {
            println!("Usage: git remote add <name> <url>");
            return Ok(());
        }
        self.name = ctx.args[0].clone();
        self.url = ctx.args[1].clone();
        
        println!("Adding remote '{}' with url '{}'", self.name, self.url);
        Ok(())
    }
}

// --- Sub-command: git remote ---
struct Remote {
    verbose: bool,
    v_flag: Flag<bool>,
}

impl Remote {
    fn new() -> Self {
        Self {
            verbose: false,
            v_flag: Flag::new("verbose").alias("v"),
        }
    }
}

impl App for Remote {
    fn name(&self) -> &str { "remote" }

    fn flags(&self) -> Vec<&dyn FlagTrait> {
        vec![&self.v_flag]
    }
    
    fn execute(&mut self, ctx: Context) -> KoralResult<()> {
        if let Some(v) = ctx.get("verbose") {
            self.verbose = v;
        }

        // naive subcommand routing
        let clean_args = ctx.args.clone();

        if clean_args.is_empty() {
             println!("Usage: git remote <command>");
             return Ok(());
        }

        match clean_args[0].as_str() {
            "add" => {
                let sub_args = clean_args[1..].to_vec();
                let mut cmd = RemoteAdd { name: "".into(), url: "".into() };
                cmd.run(sub_args)?;
            }
            _ => println!("Unknown remote command: {}", clean_args[0]),
        }
        
        if self.verbose {
            println!("(Verbose mode active for remote)");
        }
        Ok(())
    }
}

// --- Root: git ---
struct Git {
}

impl App for Git {
    fn name(&self) -> &str { "git" }
    fn version(&self) -> &str { "2.x.x (koral-style)" }

    fn execute(&mut self, ctx: Context) -> KoralResult<()> {
        let args = ctx.args;
        if args.is_empty() {
            println!("Usage: git <command>");
            return Ok(());
        }

        match args[0].as_str() {
            "remote" => {
                let mut cmd = Remote::new();
                // Pass remaining args to subcommand
                // Note: args[0] is "remote". The rest are for subcommand.
                // But wait, `run` expects args including command if strict?
                // `App::run` implementation: 
                //    parser.parse(args)
                //    execute(ctx)
                // If we pass `args[1..]`, `Remote` receives `add` `origin` `url`.
                // `Remote` parser sees `add` (positional).
                // `Remote` execute sees `add`.
                // Checks `clean_args[0]`. It is `add`. Matches.
                // Works.
                cmd.run(args[1..].to_vec())?;
            }
            "status" => {
                println!("On branch main\nNothing to commit, working tree clean");
            }
            _ => {
                println!("git: '{}' is not a git command.", args[0]);
            }
        }
        Ok(())
    }
}

fn main() -> KoralResult<()> {
    let mut app = Git { };
    // Skip executable name
    app.run(std::env::args().skip(1).collect())
}
