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

#[cfg(test)]
mod test {
    use crate::algorythms::cryptography::enigma::utils::n_abc;

    #[test]
    fn nabc() {
        assert_eq!(n_abc(0), "A".to_string());
        assert_eq!(n_abc(26), "A".to_string());
        assert_eq!(n_abc(25), "Z".to_string());
    }
}
