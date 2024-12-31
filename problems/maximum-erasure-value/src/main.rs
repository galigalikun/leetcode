fn main() {
    assert_eq!(Solution::maximum_unique_subarray(vec![4, 2, 4, 5, 6]), 17);
    assert_eq!(
        Solution::maximum_unique_subarray(vec![5, 2, 1, 2, 5, 2, 1, 2, 5]),
        8
    );
}

struct Solution;
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let mut sum = 0;
        let mut max = 0;
        let mut left = 0;
        for right in 0..nums.len() {
            while set.contains(&nums[right]) {
                sum -= nums[left];
                set.remove(&nums[left]);
                left += 1;
            }
            sum += nums[right];
            set.insert(nums[right]);
            max = max.max(sum);
        }
        max
    }
}
