use clap::crate_name;
use log::{debug, error, info, trace};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsStr,
    fs::{create_dir_all, File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Lexicon {
    entries: Vec<Entry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    pub word: String,
    pub definition: String,
}

impl Lexicon {
    pub fn new(path: &str) -> Self {
        trace!("Creating an instance of Lexicon.");

        let path: PathBuf = PathBuf::from(&path);

        debug!("Lexicon path: {:?}", &path);

        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(error) => {
                error!("Failed to open the lexicon file: {}\n", error);

                println!("Failed to open the lexicon file: {}\n", error);

                println!("Please provide the path to the lexicon file via \"-l, --lexicon-path <lexicon>\" option.\n");

                println!("For further information, provide the \"-h, --help\" flag.\n");

                std::process::exit(1);
            }
        };

        match &path.extension().and_then(OsStr::to_str) {
            Some("json") => match serde_json::de::from_reader(file) {
                Ok(lexicon) => lexicon,
                Err(error) => {
                    error!("Failed to deserialize the (JSON) lexicon file: {}", error);

                    println!("Failed to deserialize the (JSON) lexicon file: {}", error);

                    std::process::exit(1);
                }
            },
            Some("ron") => match ron::de::from_reader(file) {
                Ok(lexicon) => lexicon,
                Err(error) => {
                    error!("Failed to deserialize the (RON) lexicon file: {}", error);

                    println!("Failed to deserialize the (RON) lexicon file: {}", error);

                    std::process::exit(1);
                }
            },
            Some("toml") => {
                let mut buffer = String::new();
                file.read_to_string(&mut buffer).unwrap();

                match toml::de::from_str(buffer.as_str()) {
                    Ok(lexicon) => lexicon,
                    Err(error) => {
                        error!("Failed to deserialize the (TOML) lexicon file: {}", error);

                        println!("Failed to deserialize the (TOML) lexicon file: {}", error);

                        std::process::exit(1);
                    }
                }
            }
            _ => {
                error!("Could not deserialize the lexicon.");

                panic!("Could not deserialize the lexicon.")
            }
        }
    }

    pub fn generate() {
        trace!("Generating the default lexicon.");

        let mut lexicon_path: PathBuf = [dirs::config_dir().unwrap(), PathBuf::from(crate_name!())]
            .iter()
            .collect();

        trace!("Set the lexicon path.");

        match create_dir_all(&lexicon_path) {
            Ok(()) => info!("Successfully created/verified the directories."),
            Err(error) => error!("{}", error),
        }

        lexicon_path.push("lexicon");
        lexicon_path.set_extension("ron");

        if lexicon_path.exists() {
            info!("Lexicon file already exists.");
        } else {
            let lexicon: &'static str = include_str!("data/lexicon.ron");

            OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(&lexicon_path)
                .expect("Could not create the lexicon file.")
                .write_all(&lexicon.as_bytes())
                .unwrap();

            info!("Lexicon file generated successfully.")
        }
    }

    pub fn get_random_entry(&self) -> Option<&self::Entry> {
        trace!("Fetching a random entry from the lexicon...");

        self.entries.choose(&mut rand::thread_rng())
    }
}
