pub enum Command {
    Augment,
    Data,
    Continue,
    Help,
    Quit,
    Restart,
    Unveil,
}

impl Command {
    pub fn evaluate(command: &str) -> Self {
        match command {
            ":a" | ":augment" => Self::Augment,
            ":d" | ":data" => Self::Data,
            ":h" | ":help" => Self::Help,
            ":q" | ":quit" => Self::Quit,
            ":r" | ":restart" => Self::Restart,
            ":u" | ":unveil" => Self::Unveil,
            _ => Self::Continue,
        }
    }
}
