fn main() {
    assert_eq!(Solution::is_possible(vec![1, 2, 3, 3, 4, 5]), true);
    assert_eq!(Solution::is_possible(vec![1, 2, 3, 3, 4, 4, 5, 5]), true);
    assert_eq!(Solution::is_possible(vec![1, 2, 3, 4, 4, 5]), false);
}


// https://www.tutorialspoint.com/split-array-into-consecutive-subsequences-in-cplusplus
struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for n in nums.clone() {
            *map.entry(n).or_insert(0) += 1;
        }
        let mut cnt = nums.len() as i32;
        for n in nums {
            let mut key = n;

            if map.get(&key) > Some(&0)
                && map.get(&(key + 1)) > Some(&0)
                && map.get(&(key + 2)) > Some(&0)
            {
                *map.entry(key).or_insert(0) -= 1;
                *map.entry(key + 1).or_insert(0) -= 1;
                *map.entry(key + 2).or_insert(0) -= 1;
                key += 3;
                cnt -= 3;
                while map.get(&key) > Some(&0) && map.get(&key) > map.get(&(key - 1)) {
                    *map.entry(key).or_insert(0) -= 1;
                    key += 1;
                    cnt -= 1;
                }
            }
        }
        return if cnt == 0 { true } else { false };
    }
}
