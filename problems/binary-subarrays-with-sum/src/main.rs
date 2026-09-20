fn main() {
    assert_eq!(Solution::num_subarrays_with_sum(vec![1,0,1,0,1], 2), 4);
    assert_eq!(Solution::num_subarrays_with_sum(vec![0,0,0,0,0], 0), 15);
}

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut count_by_prefix = HashMap::new();
        count_by_prefix.insert(0, 1_i32);

        let mut prefix_sum = 0_i32;
        let mut answer = 0_i32;

        for value in nums {
            prefix_sum += value;

            if let Some(count) = count_by_prefix.get(&(prefix_sum - goal)) {
                answer += *count;
            }

            let current = count_by_prefix.get(&prefix_sum).copied().unwrap_or(0);
            count_by_prefix.insert(prefix_sum, current + 1);
        }

        answer
    }
}
