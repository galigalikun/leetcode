fn main() {
    assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    assert_eq!(Solution::majority_element(vec![3, 3, 4]), 3);
}

pub struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for n in nums {
            if let Some(x) = map.get_mut(&n) {
                *x += 1;
            } else {
                map.insert(n, 1);
            }
        }
        let mut max = 0;
        let mut result = 0;
        for (k, v) in map {
            if v > max {
                max = v;
                result = k;
            }
        }
        return result;
    }
}
