use clap::{crate_name, Clap};
use crossterm::style::{Attribute, Color};
use dirs;
use log::{debug, error, info, trace, warn};
use ron;
use serde_json;
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table, TableStyle,
};
use toml;

use std::{
    fmt::{Display, Formatter, Result},
    fs::{create_dir_all, File, OpenOptions},
    io::{stdin, stdout, Read, Write},
    path::PathBuf,
    process::Command,
    thread,
    time::Duration,
};

mod commands;
mod configuration;
mod constants;
mod cryptography;
mod gallows;
mod input;
mod lexicon;
mod settings;
mod state;
mod stylizer;
mod terminal;

use commands::Commands;
use configuration::Configuration;
use constants::Constants;
use cryptography::encrypt;
use gallows::Gallows;
use input::{retry, Input};
use lexicon::Lexicon;
use settings::Settings;
use state::{Condition, State};
use stylizer::stylize;

pub struct Game {
    pub lexicon_path: String,
    pub word: String,
    pub definition: String,
    pub secret: String,
    pub constants: Constants,
    pub commands: Vec<&'static str>,
    pub settings: Settings,
    pub gallows: Gallows,
    pub input: Input,
    pub phase: usize,
    pub round: u8,
    pub state: State,
}

impl Game {
    pub fn new() -> Self {
        let settings = Settings::parse();

        trace!("Parsed the settings.");

        Lexicon::generate();

        let lexicon_path: String = match settings.lexicon_path.clone() {
            Some(lexicon_path) => lexicon_path,
            None => {
                let mut lexicon_path: PathBuf =
                    [dirs::config_dir().unwrap(), PathBuf::from(crate_name!())]
                        .iter()
                        .collect();

                trace!("Set the lexicon path.");

                match create_dir_all(&lexicon_path) {
                    Ok(()) => info!("Successfully created/verified the directories."),
                    Err(error) => error!("{}", error),
                }

                lexicon_path.push("lexicon");
                lexicon_path.set_extension("ron");

                lexicon_path.to_str().unwrap().to_string()
            }
        };

        let lexicon = Lexicon::new(&lexicon_path);

        let entry = lexicon.get_random_entry().unwrap();
        let word = entry.word.to_lowercase().trim().to_string();
        let phase = settings.phase;

        trace!("Fetched the entry from the lexicon.");

        trace!("Commencing game creation...");

        Self {
            lexicon_path,
            word: word.clone(),
            definition: entry.definition.clone(),
            secret: encrypt(&word),
            constants: Constants::new(),
            commands: Commands::new(),
            settings,
            gallows: Gallows::new(),
            input: Input::new(),
            phase,
            round: 1,
            state: State::Initial,
        }
    }

    pub fn run(&mut self) {
        self.start();

        trace!("Successfully executed the start method.");

        self.render();

        self.update();
    }

    fn start(&mut self) {
        let mut configuration_path: PathBuf =
            [dirs::config_dir().unwrap(), PathBuf::from(crate_name!())]
                .iter()
                .collect();

        trace!("Set the configuration path.");

        match create_dir_all(&configuration_path) {
            Ok(()) => info!("Successfully created/verified the directories."),
            Err(error) => error!("{}", error),
        }

        configuration_path.push("configuration");
        configuration_path.set_extension(self.settings.extension.as_str());

        if self.settings.configure {
            let configuration = Configuration::new(
                self.settings.augment,
                self.settings.extension.clone(),
                self.lexicon_path.clone(),
                self.settings.phase,
                self.settings.unveil,
            );

            let configuration = match configuration.extension.as_str() {
                "json" => serde_json::ser::to_string_pretty(&configuration).unwrap(),
                "ron" => {
                    ron::ser::to_string_pretty(&configuration, ron::ser::PrettyConfig::default())
                        .unwrap()
                }
                "toml" => toml::ser::to_string_pretty(&configuration).unwrap(),
                _ => {
                    error!("Failed to serialize the lexicon.");

                    std::process::exit(1)
                }
            };

            OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(&configuration_path)
                .expect("Could not create the configuration file.")
                .write_all(&configuration.as_bytes())
                .unwrap();
        } else if configuration_path.exists() {
            let mut configuration_file = File::open(&configuration_path).unwrap();
            let mut buffer = String::new();

            configuration_file.read_to_string(&mut buffer).unwrap();

            let configuration: Configuration = match self.settings.extension.as_str() {
                "json" => serde_json::de::from_str(&mut buffer).unwrap(),
                "ron" => ron::de::from_str(&mut buffer).unwrap(),
                "toml" => toml::de::from_str(&mut buffer).unwrap(),
                _ => {
                    error!("Failed to deserialize the lexicon.");

                    std::process::exit(1)
                }
            };

            self.settings.augment = configuration.augment;
            self.settings.unveil = configuration.unveil;
            self.phase = configuration.phase;
        } else {
            let mut lexicon_path: PathBuf =
                [dirs::config_dir().unwrap(), PathBuf::from(crate_name!())]
                    .iter()
                    .collect();

            match create_dir_all(&lexicon_path) {
                Ok(()) => info!("Successfully created/verified the directories."),
                Err(error) => error!("{}", error),
            }

            lexicon_path.push("lexicon");
            lexicon_path.set_extension(&self.settings.extension);

            let configuration = Configuration::new(
                true,
                self.settings.extension.clone(),
                lexicon_path.to_str().unwrap().to_string(),
                self.settings.phase,
                true,
            );

            self.settings.augment = configuration.augment;
            self.settings.unveil = configuration.unveil;
            self.phase = configuration.phase;

            let configuration = match configuration.extension.as_str() {
                "json" => serde_json::ser::to_string_pretty(&configuration).unwrap(),
                "ron" => {
                    ron::ser::to_string_pretty(&configuration, ron::ser::PrettyConfig::default())
                        .unwrap()
                }
                "toml" => toml::ser::to_string_pretty(&configuration).unwrap(),
                _ => {
                    error!("Failed to serialize the lexicon.");

                    std::process::exit(1)
                }
            };

            OpenOptions::new()
                .write(true)
                .create(true)
                .open(&configuration_path)
                .expect("Could not create the configuration file.")
                .write_all(&configuration.as_bytes())
                .unwrap();
        }

        debug!("Configured phase: {}", self.phase);

        if self.phase >= self.constants.MAXIMUM_PHASE {
            warn!("Phase value exceeds the constraint.");

            self.phase = 0;
        }

        trace!("Finished the configuration phase.");

        trace!("Switching to the processing state.");

        debug!("State: {:?}", &self.state);

        self.state = State::Processing;
    }

    fn render(&self) {
        trace!("Rendering...");

        terminal::clear().unwrap();

        println!("{}", self.gallows.depict(self.phase));
        println!(
            "{}: {} ({})",
            stylize(
                "Secret",
                Attribute::Bold,
                Color::Magenta,
                self.settings.augment
            ),
            self.secret,
            self.secret.len()
        );
        println!(
            "{}: {}",
            stylize(
                "\nDefinition",
                Attribute::Bold,
                Color::Green,
                self.settings.augment
            ),
            self.definition
        );
        println!(
            "{}: {}",
            stylize(
                "\nLives",
                Attribute::Bold,
                Color::Yellow,
                self.settings.augment
            ),
            self.constants.MAXIMUM_PHASE - self.phase
        );
        println!(
            "{}: {:?} ({})",
            stylize(
                "Memory",
                Attribute::Bold,
                Color::DarkRed,
                self.settings.augment
            ),
            self.input.memory,
            self.input.memory.len()
        );
        println!(
            "{}: {}",
            stylize("Round", Attribute::Bold, Color::Blue, self.settings.augment),
            self.round
        );
    }

    fn update(&mut self) {
        trace!("Starting the game loop via the update method.");

        debug!("State: {:?}", &self.state);

        while self.state == State::Processing {
            let mut buffer = String::new();

            print!(
                "\n{}: ",
                stylize(
                    "Enter your guess",
                    Attribute::Bold,
                    Color::Cyan,
                    self.settings.augment
                )
            );

            match stdout().flush() {
                Ok(_) => (),
                Err(error) => {
                    error!("Failed to flush the screen: {}", error);

                    break;
                }
            };

            match stdin().read_line(&mut buffer) {
                Ok(_) => (),
                Err(error) => {
                    error!("Failed to read the line: {}", error);

                    break;
                }
            }

            self.input.depiction = buffer.trim().to_string().to_lowercase();
            self.input.discovered = false;
            self.input.validate();

            if self.commands.contains(&self.input.depiction.as_str()) {
                match self.input.depiction.as_str() {
                    ":data" => {
                        terminal::clear().unwrap();

                        println!("{}", &self);

                        thread::sleep(Duration::from_secs(self.constants.SECONDS_ASLEEP));

                        continue;
                    }
                    ":help" => {
                        terminal::clear().unwrap();

                        let output =
                            match Command::new(env!("CARGO_PKG_NAME")).args(&["-h"]).output() {
                                Ok(output) => output,
                                Err(error) => {
                                    error!("{}", error);

                                    std::process::exit(1);
                                }
                            };

                        std::io::stdout().write_all(&output.stdout).unwrap();

                        thread::sleep(Duration::from_secs(self.constants.SECONDS_ASLEEP));

                        continue;
                    }
                    _ => (),
                }
            } else if !self.input.valid {
                println!(
                    "{}",
                    stylize(
                        "\nInvalid input!",
                        Attribute::Bold,
                        Color::DarkYellow,
                        self.settings.augment
                    )
                );

                thread::sleep(Duration::from_secs(self.constants.SECONDS_ASLEEP));

                continue;
            }

            match self.input.depiction.len() {
                1 => {
                    for (index, letter) in self.word.to_lowercase().chars().enumerate() {
                        if self.input.initial() == letter {
                            self.secret = format!(
                                "{}{}{}",
                                &self.secret[0..index],
                                self.input.initial(),
                                &self.secret[index + 1..]
                            );

                            trace!("The player guessed a single letter.");

                            self.input.discovered = true;
                        }
                    }
                }
                _ => {
                    if self.input.depiction == self.word {
                        self.secret = self.word.clone();
                        self.input.memory.push(self.secret.clone());
                        self.render();

                        self.state = State::Resolved;

                        debug!("State: {:?}", &self.state);

                        self.state.evaluate(Condition::Won, self.settings.augment);

                        thread::sleep(Duration::from_secs(self.constants.SECONDS_ASLEEP));

                        info!("The player guessed the whole word.");

                        info!("Data:\n{}", &self);

                        if retry(self.settings.augment) {
                            info!("Restarting the game...");

                            Game::new().run();
                        }

                        break;
                    }
                }
            }

            self.input.memory.push(self.input.depiction.clone());

            if !self.input.discovered {
                self.phase += 1;

                trace!("Transitioning to the following phase.")
            }

            if self.word == self.secret {
                self.render();

                self.state = State::Resolved;

                debug!("State: {:?}", &self.state);

                self.state.evaluate(Condition::Won, self.settings.augment);

                thread::sleep(Duration::from_secs(self.constants.SECONDS_ASLEEP));

                info!("The player guessed the word letter by letter.");

                info!("Data:\n{}", &self);

                if retry(self.settings.augment) {
                    info!("Restarting the game...");

                    Game::new().run();
                }

                break;
            } else if self.phase == self.constants.MAXIMUM_PHASE {
                self.render();

                self.state = State::Resolved;

                debug!("State: {:?}", &self.state);

                self.state.evaluate(Condition::Lost, self.settings.augment);

                if self.settings.unveil {
                    println!(
                        "{}: {}\n",
                        stylize(
                            "Secret word",
                            Attribute::Bold,
                            Color::White,
                            self.settings.augment
                        ),
                        stylize(
                            self.word.as_str(),
                            Attribute::Bold,
                            Color::DarkGreen,
                            self.settings.augment
                        )
                    );
                }

                thread::sleep(Duration::from_secs(self.constants.SECONDS_ASLEEP));

                info!("The player failed to guessed the word.");

                info!("Data:\n{}", &self);

                if retry(self.settings.augment) {
                    info!("Restarting the game...");

                    Game::new().run();
                }

                break;
            }

            self.round += 1;

            trace!("Finished the current round.");

            self.render();
        }
    }
}

impl Display for Game {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let mut table = Table::new();

        table.max_column_width = self.constants.MAXIMUM_COLUMN_WIDTH;

        table.style = TableStyle::extended();

        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            "Data",
            2,
            Alignment::Center,
        )]));

        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Word", 1, Alignment::Center),
            TableCell::new_with_alignment(&self.word, 1, Alignment::Center),
        ]));

        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Definition", 1, Alignment::Center),
            TableCell::new_with_alignment(&self.definition, 1, Alignment::Center),
        ]));

        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Secret", 1, Alignment::Center),
            TableCell::new_with_alignment(&self.secret, 1, Alignment::Center),
        ]));

        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Memory", 1, Alignment::Center),
            TableCell::new_with_alignment(
                format!("{:?}", &self.input.memory),
                1,
                Alignment::Center,
            ),
        ]));

        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Lives", 1, Alignment::Center),
            TableCell::new_with_alignment(
                self.constants.MAXIMUM_PHASE - &self.phase,
                1,
                Alignment::Center,
            ),
        ]));

        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Round", 1, Alignment::Center),
            TableCell::new_with_alignment(&self.round, 1, Alignment::Center),
        ]));

        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("State", 1, Alignment::Center),
            TableCell::new_with_alignment(format!("{:?}", &self.state), 1, Alignment::Center),
        ]));

        write!(formatter, "{}", table.render())
    }
}
