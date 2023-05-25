pub use super::super::Solution;

use std::collections::HashMap;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let cache: HashMap<i32, Vec<String>> = HashMap::new();
        Solution::generate_cached_parenthesis(&n, std::rc::Rc::new(std::cell::RefCell::new(cache)))
    }
    fn generate_cached_parenthesis(
        n: &i32,
        cache: std::rc::Rc<std::cell::RefCell<HashMap<i32, Vec<String>>>>,
    ) -> Vec<String> {
        if cache.borrow().contains_key(n) {
            cache.borrow().get(&n).unwrap().clone()
        } else {
            let mut out = if n > &1 {
                [
                    (1..=n / 2).fold(vec![], |mut acc, i: i32| {
                        let fist =
                            Solution::generate_cached_parenthesis(&i, std::rc::Rc::clone(&cache)); // => ()(), (())
                        let second = Solution::generate_cached_parenthesis(
                            &(n - i),
                            std::rc::Rc::clone(&cache),
                        ); // => ()()(), (())()
                        fist.iter().for_each(|item_one: &String| {
                            second.iter().for_each(|item_two| {
                                acc.append(&mut vec![
                                    item_one.to_owned() + item_two,
                                    item_two.to_owned() + item_one,
                                ])
                            })
                        });
                        acc
                    }),
                    Solution::generate_cached_parenthesis(&(n - 1), std::rc::Rc::clone(&cache))
                        .iter_mut()
                        .fold::<Vec<String>, _>(vec![], |mut acc, x| {
                            acc.push(format!("{}()", x));
                            acc.push(format!("(){}", x));
                            acc.push(format!("({})", x));
                            acc
                        }),
                ]
                .concat()
            } else {
                vec!["()".to_string()]
            };

            out.sort();
            out.dedup();

            cache.borrow_mut().insert(n.clone(), out.clone());
            out
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example1() {
        let mut out = Solution::generate_parenthesis(3);
        out.sort();
        let mut expected = ["((()))", "(()())", "(())()", "()(())", "()()()"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        expected.sort();
        assert_eq!(out, expected);
    }
    #[test]
    fn example3() {
        let mut out = Solution::generate_parenthesis(4);
        out.sort();
        let mut expected = [
            "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
            "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        expected.sort();
        assert_eq!(out, expected);
    }
    #[test]
    fn example5() {
        let mut out = Solution::generate_parenthesis(5);
        out.sort();
        let mut expected = [
            "((((()))))",
            "(((()())))",
            "(((())()))",
            "(((()))())",
            "(((())))()",
            "((()(())))",
            "((()()()))",
            "((()())())",
            "((()()))()",
            "((())(()))",
            "((())()())",
            "((())())()",
            "((()))(())",
            "((()))()()",
            "(()((())))",
            "(()(()()))",
            "(()(())())",
            "(()(()))()",
            "(()()(()))",
            "(()()()())",
            "(()()())()",
            "(()())(())",
            "(()())()()",
            "(())((()))",
            "(())(()())",
            "(())(())()",
            "(())()(())",
            "(())()()()",
            "()(((())))",
            "()((()()))",
            "()((())())",
            "()((()))()",
            "()(()(()))",
            "()(()()())",
            "()(()())()",
            "()(())(())",
            "()(())()()",
            "()()((()))",
            "()()(()())",
            "()()(())()",
            "()()()(())",
            "()()()()()",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        expected.sort();
        assert_eq!(out, expected);
    }

    #[test]
    fn get_preset() {
        (0..=8).for_each(|n| {
            println!("{}:  {:?}", n, Solution::generate_parenthesis(n));
        })
    }
}
