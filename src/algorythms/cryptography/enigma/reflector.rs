use super::utils::n_abc;
use std::fmt::Debug;
#[derive(Clone)]
pub enum ReflectorWiring {
    A,
    B,
    C,
    // Bthin,
    // Cthin
}

impl Into<Reflector> for ReflectorWiring {
    fn into(self) -> Reflector {
        match self {
            Self::A => Reflector::new("EJMZALYXVBWFCRQUONTSPIKHGD", "A").unwrap(),
            Self::B => Reflector::new("YRUHQSLDPXNGOKMIEBFZCWVJAT", "B").unwrap(),
            Self::C => Reflector::new("FVPJIAOYEDRZXWGCTKUQSBNMHL", "C").unwrap(),
        }
    }
}

pub struct Reflector {
    pub configuration: [char; 26],
    name: String,
    model_name: Option<String>,
    date_introduced: Option<String>,
}

impl Debug for Reflector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.model_name, &self.date_introduced) {
            (Some(model_name), Some(date_introduced)) => {
                write!(
                    f,
                    "Reflector <{} ({}) from {}>",
                    self.name, model_name, date_introduced
                )
            }
            (Some(model_name), None) => {
                write!(f, "Reflector <{} ({})>", self.name, model_name)
            }
            (None, Some(date_introduced)) => {
                write!(f, "Reflector <{} from {}>", self.name, date_introduced)
            }
            _ => {
                write!(f, "Reflector <{}>", self.name)
            }
        }
    }
}

impl Reflector {
    pub fn new(wires: &str, name: &str) -> Option<Reflector> {
        match wires
            .to_ascii_lowercase()
            .chars()
            .collect::<Vec<char>>()
            .try_into()
        {
            Ok(configuration) => Some(Reflector {
                configuration,
                name: name.to_string(),
                model_name: None,
                date_introduced: None,
            }),
            _ => None,
        }
    }
    pub fn translate(&self, input: char) -> Option<char> {
        let offset = 'a' as u8;
        let limit = 'z' as u8;
        let input = input.to_ascii_lowercase() as u8;

        println!(
            "reflecting {}. {}=>{}",
            self.name,
            n_abc((input - offset) as usize),
            self.configuration[(input - offset) as usize].to_ascii_uppercase(),
        );

        if input >= offset && input <= limit {
            Some(self.configuration[(input - offset) as usize])
        } else {
            None
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
