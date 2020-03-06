use ron::de::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Gallows {
    phases: Vec<Phases>,
}

#[derive(Debug, Deserialize)]
pub struct Phases {
    pub lives: u8,
    pub depiction: String,
}

impl Gallows {
    pub fn new() -> Self {
        let gallows: &'static str = include_str!("data/gallows.ron");

        match from_str(gallows) {
            Ok(gallows) => gallows,
            Err(error) => {
                println!("Failed to load the gallows file: {}", error);

                std::process::exit(1);
            }
        }
    }

    pub fn depict(&self, phase: usize) -> String {
        self.phases[phase].depiction.clone()
    }
}
