pub use super::super::Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];

        for token in tokens {
            if let Ok(number) = token.parse::<i32>() {
                stack.push(number)
            } else {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                stack.push(match token.as_str() {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,
                    _ => unreachable!(),
                });
            }
        }
        stack.pop().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::eval_rpn(
                vec!["2", "1", "+", "3", "*"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            ),
            9
        )
    }
    #[test]
    fn example2() {
        assert_eq!(
            Solution::eval_rpn(
                vec!["4", "13", "5", "/", "+"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            ),
            6
        )
    }
    #[test]
    fn example3() {
        assert_eq!(
            Solution::eval_rpn(
                vec!["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            ),
            22
        );
    }
}
