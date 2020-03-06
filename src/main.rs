use log::trace;

mod logger;

use core::Game;
use logger::setup_logger;

fn main() {
    match setup_logger() {
        Ok(_) => {
            trace!("Initialized the logger.");

            let mut game = Game::new();

            trace!("Initialized the game.");

            trace!("Running the game...");

            game.run();
        }
        Err(error) => println!("Failed to initialize the logger: {}", error),
    };
}
