fn main() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
}

pub struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // let mut result = Vec::new();
        let mut map: HashMap<(usize, usize), i32> = HashMap::new();
        let mut day = 1;
        loop {
            if day + 1 > prices.len() {
                break;
            }
            for i in day..prices.len() - 1 {
                let diff = prices[i] - prices[day - 1];
                if diff > 0 {
                    map.insert((day - 1, i), diff);
                }
            }

            day += 1;
        }

        for ((i, j), value) in map {
            println!("{} - {} value {:?}", i, j, value);
        }
        return max;
    }
}
