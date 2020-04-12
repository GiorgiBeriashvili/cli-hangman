pub enum Command {
    Data,
    Continue,
    Help,
    Restart,
    Quit,
}

impl Command {
    pub fn evaluate(command: &str) -> Self {
        match command {
            ":d" | ":data" => Self::Data,
            ":h" | ":help" => Self::Help,
            ":r" | ":restart" => Self::Restart,
            ":q" | ":quit" => Self::Quit,
            _ => Self::Continue,
        }
    }
}
