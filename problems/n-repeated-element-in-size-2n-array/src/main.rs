fn main() {
    assert_eq!(Solution::repeated_n_times(vec![1, 2, 3, 3]), 3);
    assert_eq!(Solution::repeated_n_times(vec![2, 1, 2, 5, 3, 2]), 2);
    assert_eq!(Solution::repeated_n_times(vec![5, 1, 5, 2, 5, 3, 5, 4]), 5);
}

struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for n in nums {
            *map.entry(n).or_insert(1) += 1;
        }
        let mut ans = 0;
        let mut n = 0;
        for m in map {
            if m.1 > n {
                ans = m.0;
                n = m.1;
            }
        }
        return ans;
    }
}
