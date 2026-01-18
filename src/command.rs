#[derive(Debug, Clone)]
pub struct CommandDef {
    pub name: String,
    pub description: String,
}

impl CommandDef {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
        }
    }
}
