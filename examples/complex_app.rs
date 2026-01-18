use koral::flag::FlagDef;
use koral::traits::App;
use koral::{Context, Flag, KoralResult};

struct NameFlag;
impl Flag for NameFlag {
    type Value = String;
    fn name() -> &'static str {
        "name"
    }
}

struct UrlFlag;
impl Flag for UrlFlag {
    type Value = String;
    fn name() -> &'static str {
        "url"
    }
}

struct VerboseFlag;
impl Flag for VerboseFlag {
    type Value = bool;
    fn name() -> &'static str {
        "verbose"
    }
    fn short() -> Option<char> {
        Some('v')
    }
    fn takes_value() -> bool {
        false
    }
}

// --- Sub-command: git remote add <name> <url> ---
struct RemoteAdd {
    // args via flags for simplicity in this example
    name_arg: String,
    url_arg: String,
}

impl RemoteAdd {
    fn new() -> Self {
        Self {
            name_arg: String::new(),
            url_arg: String::new(),
        }
    }
}

impl App for RemoteAdd {
    fn name(&self) -> &str {
        "add"
    }

    fn flags(&self) -> Vec<FlagDef> {
        vec![
            FlagDef::from_trait::<NameFlag>(),
            FlagDef::from_trait::<UrlFlag>(),
        ]
    }

    fn execute(&mut self, ctx: Context) -> KoralResult<()> {
        if ctx.args.len() < 2 {
            println!("Usage: git remote add <name> <url>");
            // For example purposes we don't return error
            return Ok(());
        }

        self.name_arg = ctx.args[0].clone();
        self.url_arg = ctx.args[1].clone();

        println!("Adding remote {} with url {}", self.name_arg, self.url_arg);
        Ok(())
    }
}

// --- Sub-command: git remote ---
struct Remote {
    verbose: bool,
}

impl Remote {
    fn new() -> Self {
        Self { verbose: false }
    }
}

impl App for Remote {
    fn name(&self) -> &str {
        "remote"
    }

    fn flags(&self) -> Vec<FlagDef> {
        vec![FlagDef::from_trait::<VerboseFlag>()]
    }

    fn execute(&mut self, ctx: Context) -> KoralResult<()> {
        if ctx.get::<VerboseFlag>().unwrap_or(false) {
            self.verbose = true;
        }

        if !ctx.args.is_empty() {
            // Manual subcommand routing for "add"
            if ctx.args[0] == "add" {
                let mut add_cmd = RemoteAdd::new();
                // Pass remaining args
                let remaining_args = ctx.args[1..].to_vec();
                // We need to run the sub-app.
                // App::run expects full args list usually starting with program name?
                // No, standard App::run takes args directly.
                return add_cmd.run(remaining_args);
            }
        }

        println!("Listing remotes...");
        if self.verbose {
            println!("origin  https://github.com/user/repo (fetch)");
            println!("origin  https://github.com/user/repo (push)");
        } else {
            println!("origin");
        }
        Ok(())
    }

    fn subcommands(&self) -> Vec<&dyn App> {
        // We can expose subcommands for help generation if we stored them
        // But here we do manual routing in execute, so we return empty?
        // Or we can construct them just for help?
        // Let's keep it simple.
        vec![]
    }
}

// --- Root: git ---
struct Git {}

impl App for Git {
    fn name(&self) -> &str {
        "git"
    }
    fn version(&self) -> &str {
        "2.x.x (koral-style)"
    }

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
    let mut app = Git {};
    // Skip executable name
    app.run(std::env::args().skip(1).collect())
}
