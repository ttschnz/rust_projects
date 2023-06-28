use std::{cell::RefCell, fmt::Debug, rc::Rc};

use super::utils::n_abc;

#[derive(Clone, Debug)]
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
            Self::Ic => Rotor::new(
                [
                    3, 12, 19, 22, 18, 8, 11, 17, 20, 24, 16, 13, 10, 5, 4, 9, 2, 0, 25, 1, 15, 6,
                    23, 14, 7, 21,
                ],
                "Ic",
            ),
            Self::Iic => Rotor::new(
                [
                    7, 16, 25, 6, 15, 9, 19, 12, 14, 1, 11, 13, 2, 8, 5, 3, 24, 0, 22, 21, 4, 20,
                    18, 17, 10, 23,
                ],
                "Iic",
            ),
            Self::Iiic => Rotor::new(
                [
                    20, 16, 13, 19, 11, 18, 25, 5, 12, 17, 4, 7, 3, 15, 23, 10, 8, 1, 21, 24, 6, 9,
                    2, 22, 14, 0,
                ],
                "Iiic",
            ),
            Self::I => Rotor::new(
                [
                    4, 10, 12, 5, 11, 6, 3, 16, 21, 25, 13, 19, 14, 22, 24, 7, 23, 20, 18, 15, 0,
                    8, 1, 17, 2, 9,
                ],
                "I",
            ),
            Self::Ii => Rotor::new(
                [
                    0, 9, 3, 10, 18, 8, 17, 20, 23, 1, 11, 7, 22, 19, 12, 2, 16, 6, 25, 13, 15, 24,
                    5, 21, 14, 4,
                ],
                "Ii",
            ),
            Self::Iii => Rotor::new(
                [
                    1, 3, 5, 7, 9, 11, 2, 15, 17, 19, 23, 21, 25, 13, 24, 4, 8, 22, 6, 0, 10, 12,
                    20, 18, 16, 14,
                ],
                "Iii",
            ),
        }
    }
}

pub struct Rotor {
    pub wires: [usize; 26],
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
    pub fn new(wires: [usize; 26], name: &str) -> Rotor {
        Rotor {
            wires,
            name: name.to_string(),
            model_name: None,
            date_introduced: None,
            rotations: 0,
            carry_rotor: None,
        }
    }
    pub fn forward(&self, input: usize) -> usize {
        let index = (input + self.rotations as usize) & self.wires.len();
        self.wires[index]
    }

    pub fn backward(&self, input: usize) -> Option<usize> {
        // input is value at certain index. this index - rotations is the output
        // however, we need to do some kind of modulo. but i don't know where yet.

        if let Some((index, _)) = self.wires.iter().enumerate().find(|(_, c)| *c == &(input)) {
            Some(index - self.rotations as usize)
        } else {
            println!("could not find {} in {:?}", input, self.wires);
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
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
