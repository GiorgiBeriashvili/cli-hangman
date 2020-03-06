use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
    pub augment: bool,
    pub extension: String,
    pub lexicon_path: String,
    pub phase: usize,
    pub unveil: bool,
}

impl Configuration {
    pub fn new(
        augment: bool,
        extension: String,
        lexicon_path: String,
        phase: usize,
        unveil: bool,
    ) -> Self {
        Self {
            augment,
            extension,
            lexicon_path,
            phase,
            unveil,
        }
    }
}
