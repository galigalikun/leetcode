fn main() {
    assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 2, 3, 3]), 3);
    assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 3]), 2);
    assert_eq!(Solution::distribute_candies(vec![6, 6, 6, 6]), 1);
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let n = candy_type.len() / 2;
        let map = candy_type
            .into_iter()
            .map(|x| (x, 1))
            .collect::<HashMap<i32, usize>>();
        return std::cmp::min(n, map.len()) as i32;
    }
}
