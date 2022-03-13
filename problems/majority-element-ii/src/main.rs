fn main() {
    assert_eq!(Solution::majority_element(vec![3, 2, 3]), vec![3]);
    assert_eq!(Solution::majority_element(vec![1]), vec![1]);
    assert_eq!(Solution::majority_element(vec![1, 2]), vec![1, 2]);
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() / 3;
        let mut map = HashMap::new();
        for n in nums {
            if let Some(x) = map.get_mut(&n) {
                *x += 1;
            } else {
                map.insert(n, 1);
            }
        }
        let mut result = vec![];
        for (k, v) in map {
            if v > n {
                result.push(k);
            }
        }

        return result;
    }
}
