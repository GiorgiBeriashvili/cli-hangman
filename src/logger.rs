use chrono::Local;
use clap::crate_name;
use fern::{log_file, Dispatch, InitError};
use log::LevelFilter;
use std::{fs::create_dir_all, path::PathBuf};

pub fn setup_logger() -> Result<(), InitError> {
    let mut path: PathBuf = [dirs::config_dir().unwrap(), PathBuf::from(crate_name!())]
        .iter()
        .collect();

    match create_dir_all(&path) {
        Ok(()) => (),
        Err(error) => println!("{}: Failed to create/verify the directories.", error),
    }

    path.push("main");
    path.set_extension("log");

    Dispatch::new()
        .format(|output, message, record| {
            output.finish(format_args!(
                "{}[{}][{}] {}",
                Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Trace)
        .chain(log_file(&path)?)
        .apply()?;

    Ok(())
}
