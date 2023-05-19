pub use super::super::Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::HashMap;
        if nums.len() as i32 == k {
            return nums;
        }
        let mut map = HashMap::new();

        for num in nums {
            map.entry(num).and_modify(|n| *n += 1).or_insert(1);
        }

        let mut most_common: Vec<(&i32, &i32)> = vec![];
        for (num, frequency) in map.iter() {
            if most_common.len() < k as usize || frequency >= most_common[0].1 {
                // println!("adding {:?}", (num, frequency));
                most_common.push((num, frequency));
                most_common.sort_by_key(|x| x.1);
                if most_common.len() > k as usize {
                    // println!("removing from list {:?}", most_common);
                    most_common = most_common
                        .split_at(most_common.len() - k as usize)
                        .1
                        .to_vec();
                    // println!("removed from list {:?}", most_common);
                }
            } else {
                // println!("ignoring {:?}", (num, frequency));
            }
        }

        most_common.iter().map(|(n, _)| *n.clone()).collect()
    }
}
