fn main() {
    assert_eq!(Solution::max_frequency(vec![1, 4, 5], 1, 2), 2);
    assert_eq!(Solution::max_frequency(vec![5, 11, 20, 20], 5, 1), 2);
}

struct Solution;
impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut left = 0;
        let mut total = 0;
        let mut result = 1;
        if num_operations < k {
            return 0;
        }
        for right in 0..nums.len() {
            total += nums[right];
            while (nums[right] * (right - left + 1) as i32) - total > k {
                total -= nums[left];
                left += 1;
            }
            result = result.max((right - left + 1) as i32);
        }
        result
    }
}
