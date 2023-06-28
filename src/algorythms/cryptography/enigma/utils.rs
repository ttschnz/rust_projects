pub fn abc() -> String {
    "abcdefghijklmnopqrstuvwxyz"
        .to_string()
        .to_ascii_uppercase()
}

pub fn n_abc(index: usize) -> String {
    let abc = abc();
    abc.get(index % 26..=index % 26)
        .unwrap()
        .clone()
        .to_string()
}
pub fn abc_n(abc_char: char) -> usize {
    let abc = abc();
    abc.find(abc_char.to_ascii_uppercase()).unwrap()
}

#[cfg(test)]
mod test {
    use crate::algorythms::cryptography::enigma::{
        rotor::{Rotor, RotorWiring},
        utils::n_abc,
    };

    use super::abc_n;

    #[test]
    fn nabc() {
        assert_eq!(n_abc(0), "A".to_string());
        assert_eq!(n_abc(26), "A".to_string());
        assert_eq!(n_abc(25), "Z".to_string());
    }

    #[test]
    fn abcn() {
        assert_eq!(abc_n('a'), 0);
        assert_eq!(abc_n('A'), 0);
        assert_eq!(abc_n('z'), 25);
        assert_eq!(abc_n('Z'), 25);
    }

    #[test]
    fn translate_config() {
        (vec![
            RotorWiring::Ic,
            RotorWiring::Iic,
            RotorWiring::Iiic,
            RotorWiring::I,
            RotorWiring::Ii,
            RotorWiring::Iii,
        ])
        .iter()
        .for_each(|r| {
            println!(
                "Self::{:?}=>Rotor::new(vec!{:?}, \"{:?}\").unwrap(),",
                r.clone(),
                <RotorWiring as Into<Rotor>>::into(r.clone())
                    .wires
                    .map(abc_n)
                    .iter()
                    .collect::<Vec<_>>(),
                r.clone(),
            );
        })
        // for c in <RotorWiring as Into<Rotor>>::into(RotorWiring::I).configuration {
        //     out_vec.push(abc_n(c));
        // }
    }
}
