use super::super::Solution;

impl Solution {
    // pub fn is_valid_regex(s: String) -> bool {
    //     use regex::Regex;
    //     let mut s = s;
    //     let parentheses = Regex::new(r"\(\)|\[\]|\{\}").unwrap();
    //     while parentheses.is_match(s.as_str()) {
    //         s = parentheses.replace_all(s.as_str(), "").to_string()
    //     }
    //     s.len() == 0
    // }
    pub fn is_valid(s: String) -> bool {
        let mut s = s;
        let mut done = false;

        while !done {
            if s.find("()").is_some() {
                s = s.replace("()", "")
            } else if s.find("{}").is_some() {
                s = s.replace("{}", "")
            } else if s.find("[]").is_some() {
                s = s.replace("[]", "")
            } else {
                done = true
            }
        }
        s.len() == 0
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn example1() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::is_valid("(]".to_string()), false);
    }
    #[test]
    fn nested() {
        assert_eq!(Solution::is_valid("(({[{{()}}]}))".to_string()), true);
    }
}
