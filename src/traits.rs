use crate::context::Context;
use crate::flag;

pub trait App {
    fn name(&self) -> String;
    fn action(&self, ctx: Context) -> Result<(), Box<dyn std::error::Error>>;
    fn run(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    fn flags(&self) -> Vec<flag::Flag> {
        vec![]
    }
}

pub trait Flag: Clone {
    type Kind;
    type Value;

    fn kind(self) -> Self::Kind;
    fn name(self) -> String;
    fn alias(self, alias: impl Into<String>) -> Self;
    fn option_index(&self, v: &[String]) -> Option<usize>;
    fn value(&self, args: &[String]) -> Option<Self::Value>;
}

// pub trait Context<F: Flag> {
//     fn args(self) -> Vec<String>;
//     fn flags(self) -> HashMap<String, Option<F::Value>>;
// }
