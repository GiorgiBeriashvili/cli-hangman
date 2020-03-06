use crossterm::style::{Attribute, Color};

use crate::stylizer::stylize;

#[derive(Debug, PartialEq)]
pub enum State {
    Initial,
    Processing,
    Resolved,
}

impl State {
    pub fn evaluate(&self, condition: Condition, augment: bool) {
        match self {
            Self::Resolved => match condition {
                Condition::Lost => {
                    println!(
                        "{}",
                        stylize("\nYou have lost.\n", Attribute::Bold, Color::Red, augment)
                    );
                }
                Condition::Won => {
                    println!(
                        "{}",
                        stylize(
                            "\nYou have won!\n",
                            Attribute::Bold,
                            Color::DarkGreen,
                            augment
                        )
                    );
                }
            },
            _ => (),
        }
    }
}

#[derive(PartialEq)]
pub enum Condition {
    Lost,
    Won,
}
