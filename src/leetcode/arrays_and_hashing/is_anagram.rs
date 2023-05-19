use std::collections::HashMap;

pub use super::super::Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        };
        let mut map = HashMap::<char, i16>::new();
        let mut chars = s.chars();
        while let Some(char) = chars.next() {
            if let Some(entry) = map.get_mut(&char) {
                *entry += 1;
            } else {
                map.insert(char, 1);
            }
        }

        let mut chars = t.chars();
        while let Some(char) = chars.next() {
            if let Some(count_left) = map.get_mut(&char) {
                if *count_left == 0 {
                    return false;
                }
                *count_left -= 1;
            } else {
                return false;
            }
        }
        map.into_values().all(|v| v == 0)
    }
}
