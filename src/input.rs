use crossterm::{
    event::{read, Event, KeyCode},
    style::{Attribute, Color},
};
use std::io::{stdout, Write};

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
        if !self.memory.contains(&self.depiction) && !&self.depiction.trim().is_empty() {
            self.valid = true;
        } else {
            self.valid = false;
        }
    }
}

pub fn prompt_restart(augment: bool) -> bool {
    print!(
        "{} [Y/n]",
        format!(
            "{}",
            stylize(
                "Would you like to restart the game?",
                Attribute::Bold,
                Color::Yellow,
                augment
            )
        )
        .as_str()
    );

    stdout()
        .flush()
        .expect("Could not flush the standard output.");

    loop {
        match read() {
            Ok(Event::Key(event)) => match event.code {
                KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                    println!();

                    break true;
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    println!();

                    break false;
                }
                _ => continue,
            },
            _ => continue,
        };
    }
}
