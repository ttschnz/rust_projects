pub use super::super::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums: Vec<(usize, i32)> = nums.into_iter().enumerate().collect();
        nums.sort_unstable_by_key(|&(_, n)| n);
        let mut a = 0;
        let mut b = nums.len() - 1;
        let mut comparing_target;
        while a < b {
            println!("comparing and adjusting b");
            comparing_target = target - nums[a].1;
            while nums[b].1 > comparing_target {
                println!("adjusting b");
                b -= 1;
            }
            if nums[a].1 + nums[b].1 == target {
                return vec![nums[a].0 as i32, nums[b].0 as i32];
            }
            println!("comparing and adjusting a");
            comparing_target = target - nums[b].1;
            while nums[a].1 < comparing_target {
                a += 1;
            }
            if nums[a].1 + nums[b].1 == target {
                return vec![nums[a].0 as i32, nums[b].0 as i32];
            }
        }
        return vec![0, 0];
    }
}
