#![allow(non_snake_case)]
use ron::de::from_str;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Constants {
    pub MAXIMUM_COLUMN_WIDTH: usize,
    pub MAXIMUM_PHASE: usize,
    pub SECONDS_ASLEEP: u64,
}

impl Constants {
    pub fn new() -> Self {
        let constants: &'static str = include_str!("data/constants.ron");

        match from_str(constants) {
            Ok(constants) => constants,
            Err(error) => {
                println!("Failed to load the constants file: {}", error);

                std::process::exit(1);
            }
        }
    }
}
