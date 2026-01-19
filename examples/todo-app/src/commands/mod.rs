pub mod add;
pub mod completion;
pub mod done;
pub mod list;

#[derive(koral::Subcommand)]
pub enum TodoCmd {
    #[subcommand(name = "add", aliases = "a")]
    Add(add::AddCmd),
    #[subcommand(name = "list", aliases = "ls")]
    List(list::ListCmd),
    #[subcommand(name = "done", aliases = "d")]
    Done(done::DoneCmd),
    #[subcommand(name = "completion")]
    Completion(completion::CompletionCmd),
}

impl Default for TodoCmd {
    fn default() -> Self {
        Self::List(list::ListCmd::default())
    }
}
