use ron::de::from_str;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_gallows() -> Gallows {
        Gallows::new()
    }

    #[test]
    fn test_depict() {
        let gallows = setup_gallows();

        assert_eq!(
            "\n_________\n|/      |\n|       O\n|      /|\\\n|      / \\\n|\n|___\n",
            gallows.depict(6)
        );

        assert_eq!(
            "\n_________\n|/      |\n|       O\n|      /|\\\n|      /\n|\n|___\n",
            gallows.depict(5)
        );

        assert_eq!(
            "\n_________\n|/      |\n|       O\n|      /|\\\n|\n|\n|___\n",
            gallows.depict(4)
        );

        assert_eq!(
            "\n_________\n|/      |\n|       O\n|      /|\n|\n|\n|___\n",
            gallows.depict(3)
        );

        assert_eq!(
            "\n_________\n|/      |\n|       O\n|       |\n|\n|\n|___\n",
            gallows.depict(2)
        );

        assert_eq!(
            "\n_________\n|/      |\n|       O\n|\n|\n|\n|___\n",
            gallows.depict(1)
        );

        assert_eq!(
            "\n_________\n|/      |\n|\n|\n|\n|\n|___\n",
            gallows.depict(0)
        );
    }
}
