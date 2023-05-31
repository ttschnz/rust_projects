use std::{cell::RefCell, fmt::Debug, rc::Rc};

use super::utils::n_abc;

#[derive(Clone)]
pub enum RotorWiring {
    Ic,
    Iic,
    Iiic,
    I,
    Ii,
    Iii,
}

impl Into<Rotor> for RotorWiring {
    fn into(self) -> Rotor {
        match self {
            Self::Ic => Rotor::new("DMTWSILRUYQNKFEJCAZBPGXOHV", "IC").unwrap(),
            Self::Iic => Rotor::new("HQZGPJTMOBLNCIFDYAWVEUSRKX", "IIC").unwrap(),
            Self::Iiic => Rotor::new("UQNTLSZFMREHDPXKIBVYGJCWOA", "IIIC").unwrap(),
            Self::I => Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", "I").unwrap(),
            Self::Ii => Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", "II").unwrap(),
            Self::Iii => Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", "III").unwrap(),
        }
    }
}

pub struct Rotor {
    configuration: [char; 26],
    name: String,
    model_name: Option<String>,
    date_introduced: Option<String>,
    rotations: u8,
    carry_rotor: Option<Rc<RefCell<Rotor>>>,
}

impl Debug for Rotor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.model_name, &self.date_introduced) {
            (Some(model_name), Some(date_introduced)) => {
                write!(
                    f,
                    "Rotor <{} ({}) from {}, offset={}>",
                    self.name, model_name, date_introduced, self.rotations
                )
            }
            (Some(model_name), None) => {
                write!(
                    f,
                    "Rotor <{} ({}), offset={}>",
                    self.name, model_name, self.rotations
                )
            }
            (None, Some(date_introduced)) => {
                write!(
                    f,
                    "Rotor <{} from {}, offset={}>",
                    self.name, date_introduced, self.rotations
                )
            }
            _ => {
                write!(f, "Rotor <{}, offset={}>", self.name, self.rotations)
            }
        }
    }
}

impl Rotor {
    pub fn new(wires: &str, name: &str) -> Option<Rotor> {
        match wires
            .to_ascii_lowercase()
            .chars()
            .collect::<Vec<char>>()
            .try_into()
        {
            Ok(configuration) => Some(Rotor {
                configuration,
                name: name.to_string(),
                model_name: None,
                date_introduced: None,
                rotations: 0,
                carry_rotor: None,
            }),
            _ => None,
        }
    }
    pub fn forward(&self, input: char) -> Option<char> {
        let offset = 'a' as u8;
        let limit = 'z' as u8;

        let input = input.to_ascii_lowercase() as u8;
        if input >= offset && input <= limit {
            println!(
                "translate {} in {} position. {}=>{}",
                self.name,
                n_abc(self.rotations as usize),
                n_abc((input - offset) as usize),
                self.configuration[((input - offset + self.rotations) % 26) as usize]
                    .to_ascii_uppercase(),
            );
            Some(self.configuration[((input - offset + self.rotations) % 26) as usize])
        } else {
            None
        }
    }

    pub fn backward(&self, input: char) -> Option<char> {
        let offset = 'a' as u8;
        if let Some((index, _)) = self
            .configuration
            .iter()
            .enumerate()
            .find(|(_, c)| *c == &(input.to_ascii_lowercase()))
        {
            Some((index as u8 + self.rotations + offset) as char)
        } else {
            println!("could not find {} in {:?}", input, self.configuration);
            None
        }
    }

    pub fn increment(&mut self) {
        let prev = self.rotations;
        self.rotations += 1;
        println!("{} from {} to {}", self.name, prev, self.rotations);
        if self.rotations % 26 != self.rotations {
            self.rotations = self.rotations % 26;
            if let Some(other) = &self.carry_rotor {
                other.borrow_mut().increment();
            } else {
                // dropped increment
            }
        }
    }
    pub fn set_carry_rotor(&mut self, rotor: Option<Rc<RefCell<Rotor>>>) {
        self.carry_rotor = rotor.clone();
    }
}
