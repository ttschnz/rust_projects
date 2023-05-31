pub use super::super::Solution;

use std::collections::HashMap;
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut subscriptions: HashMap<i32, Vec<usize>> = HashMap::new(); // subscribe to a temperature with the current index
        let mut out = vec![];
        for (current_day, temperature) in temperatures.iter().enumerate() {
            // fulfill subscriptions to all lower temperatures
            subscriptions
                .iter_mut()
                .filter(|(k, _v)| *k < &temperature)
                .for_each(|(_key, indexes)| {
                    while let Some(index) = indexes.pop() {
                        out[index] = (current_day - index) as i32;
                    }
                });

            // add 0 to current day and subscribe to higher temperatures
            out.push(0);
            subscriptions
                .entry(*temperature)
                .and_modify(|waiting_list| waiting_list.push(current_day))
                .or_insert(vec![current_day]);
        }
        out
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn example1() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }
    #[test]
    fn example2() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        );
    }
    #[test]
    fn example3() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0]
        );
    }
}
