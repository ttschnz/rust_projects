pub use super::super::Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let map = nums.iter().collect::<std::collections::HashSet<_>>();
        let mut high_score = 0;

        for &num in &nums {
            if !map.contains(&(num - 1)) {
                high_score = high_score.max((num..).take_while(|x| map.contains(x)).count());
            }
        }

        high_score as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn finds_longest_5() {
        assert_eq!(Solution::longest_consecutive(vec![1, 2, 3, 4, 5]), 5);
    }
    #[test]
    fn finds_longest_10() {
        assert_eq!(
            Solution::longest_consecutive((1..=10).collect::<Vec<i32>>()),
            10
        );
    }
    #[test]
    fn finds_longest_100() {
        assert_eq!(
            Solution::longest_consecutive((1..=100).collect::<Vec<i32>>()),
            100
        );
    }
    #[test]
    fn finds_longest_1000() {
        assert_eq!(
            Solution::longest_consecutive((1..=1000).collect::<Vec<i32>>()),
            1000
        );
    }
}
