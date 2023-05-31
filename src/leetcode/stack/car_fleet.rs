#[allow(unused_imports)]
use itertools::rev;
#[allow(unused_imports)]
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::super::Solution;

struct Car {
    #[allow(unused)]
    speed: i32,
}

impl Solution {
    pub fn car_fleet(target: i32, positions: Vec<i32>, speed: Vec<i32>) -> i32 {
        // transpose data to Positions
        let mut tiles: HashMap<i32, Vec<Car>> = HashMap::new();
        for (position, speed) in positions.iter().zip(speed.iter()) {
            tiles
                .entry(position.clone())
                .and_modify(|t| {
                    t.push(Car { speed: *speed });
                })
                .or_insert(vec![Car { speed: *speed }]);
        }

        // while not all cars have reached the target
        while tiles.len() > 0 {
            tiles.remove(&target);
            // increment each position by speed, but check that none is overtaken
            for index in (0..target).rev() {
                tiles
                    .remove(&index)
                    .and_then(|cars| tiles.insert(index + 1, cars));
            }
            // count how many have position of target and make them +1 (target + 1 is dbd max)
        }

        todo!()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn example1() {}
}
