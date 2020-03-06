use log::trace;

pub struct Commands;

impl Commands {
    pub fn new() -> Vec<&'static str> {
        trace!("Creating an instance of Commands.");

        vec![":data", ":help"]
    }
}
