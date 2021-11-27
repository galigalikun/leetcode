fn main() {
    assert_eq!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6), true);
    assert_eq!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 6), true);
    assert_eq!(
        Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 13),
        false
    );
}

// https://dev.to/sphoorthi/leetcode-continuous-subarray-sum-459o
struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::new();
        map.insert(0, -1);
        let mut sum = 0;
        let min_len = 2;
        for i in 0..nums.len() {
            sum += nums[i];
            if k != 0 {
                sum = sum % k;
            }
            if let Some(m) = map.get(&sum) {
                if i as i32 - *m >= min_len {
                    return true;
                }
            } else {
                map.insert(sum, i as i32);
            }
        }
        return false;
    }
}
