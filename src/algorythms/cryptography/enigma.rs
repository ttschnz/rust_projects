mod formatter;
mod reflector;
mod rotor;
mod utils;
use reflector::{Reflector, ReflectorWiring};
use rotor::{Rotor, RotorWiring};

use std::{cell::RefCell, rc::Rc};

pub struct Enigma {
    rotors: Vec<Rc<RefCell<Rotor>>>,
    reflector: Rc<RefCell<Reflector>>, // plugboard
}

impl Enigma {
    pub fn new(mut rotors: Vec<RotorWiring>, reflector: ReflectorWiring) -> Enigma {
        let mut last_rotor = None;
        rotors.reverse();
        let mut rotors = rotors
            .iter()
            .map(|rotor_definition| {
                let mut rotor: Rotor = rotor_definition.clone().into();
                rotor.set_carry_rotor(last_rotor.clone());
                let rotor = Rc::new(RefCell::new(rotor));
                last_rotor = Some(rotor.clone());
                rotor
            })
            .collect::<Vec<Rc<RefCell<Rotor>>>>();
        rotors.reverse();
        Enigma {
            rotors,
            reflector: Rc::new(RefCell::new(reflector.clone().into())),
        }
    }
    pub fn encode(&mut self, input: &str) -> String {
        input.chars().fold("".to_string(), |mut acc, c| {
            // increment first rotor (and carry to next)
            if let Some(first) = self.rotors.first() {
                first.borrow_mut().increment()
            };
            println!("forward");
            // forward
            let forward_translated = self.rotors.iter().fold(Some(c), |curr, rotor| {
                curr.and_then(|curr| rotor.borrow().forward(curr))
            });
            println!("reflecting");
            // reflect
            let reflected =
                forward_translated.and_then(|curr| self.reflector.borrow().translate(curr));

            println!("backward");
            // backward
            let backward_translated = self.rotors.iter().rev().fold(reflected, |curr, rotor| {
                curr.and_then(|curr| rotor.borrow().backward(curr))
            });

            if let Some(curr) = backward_translated {
                acc.push(curr)
            }

            println!("\n\n");
            // }
            acc
        })
    }
    pub fn reset(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::algorythms::cryptography::enigma::ReflectorWiring;

    // use usper::{RotorStream};
    use super::{Enigma, RotorWiring};

    #[test]
    fn enigma() {
        // assert_eq!(('A' as u8 - 65) as usize, 0);

        let mut enigma = Enigma::new(
            vec![RotorWiring::Iii, RotorWiring::Ii, RotorWiring::I],
            ReflectorWiring::B,
        );
        println!("{:?}", enigma);
        // println!("{}", enigma.encode("a").to_ascii_uppercase());
        // println!("{:?}", enigma.rotors);
    }
}
