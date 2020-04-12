use crossterm::style::{Attribute, Color};
use dialoguer::Confirmation;

use crate::stylizer::stylize;

#[derive(Debug, Default)]
pub struct Input {
    pub depiction: String,
    pub discovered: bool,
    pub memory: Vec<String>,
    pub valid: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            depiction: String::new(),
            discovered: false,
            memory: vec![],
            valid: false,
        }
    }

    pub fn initial(&self) -> char {
        match self.depiction.chars().next() {
            Some(character) => character,
            None => '-',
        }
    }

    pub fn validate(&mut self) {
        if !self.memory.contains(&self.depiction) {
            self.valid = true;
        } else {
            self.valid = false;
        }
    }
}

pub fn retry(augment: bool) -> bool {
    Confirmation::new()
        .with_text(
            format!(
                "{}",
                stylize(
                    "Would you like to restart the game?",
                    Attribute::Bold,
                    Color::Yellow,
                    augment
                )
            )
            .as_str(),
        )
        .interact()
        .unwrap()
}
