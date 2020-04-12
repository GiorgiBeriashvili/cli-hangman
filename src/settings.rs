use clap::{crate_authors, crate_description, crate_name, crate_version, Clap};
use serde::{Deserialize, Serialize};

#[derive(Clap, Debug, Default, Serialize, Deserialize)]
#[clap(about = crate_description!(), author = crate_authors!(), name = crate_name!(), version = crate_version!())]
pub struct Settings {
    /// Augments the output (applies predefined style)
    #[clap(name = "augment", short, long)]
    pub augment: bool,

    /// Saves the current configuration
    ///
    /// Conflicts with the "default" flag. The configuration is saved on the
    /// disk, inside the user's default configuration directory, in the
    /// cli-hangman/configuration.ron file
    #[clap(name = "configure", short, long)]
    pub configure: bool,

    /// Sets the preferred extension for the default configuration/lexicon files
    ///
    /// Supported extensions: [json, ron, toml]
    #[clap(name = "extension", short, long, default_value = "ron")]
    pub extension: String,

    /// Sets the lexicon path
    #[clap(name = "lexicon-path", short = "l", long)]
    pub lexicon_path: Option<String>,

    /// Sets the starting phase value
    #[clap(name = "phase", short, long, default_value = "0")]
    pub phase: usize,

    /// Unveils the secret at the end of the losing session
    #[clap(name = "unveil", short, long)]
    pub unveil: bool,
}
