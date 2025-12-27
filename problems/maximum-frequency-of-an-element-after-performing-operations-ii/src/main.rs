fn main() {
    assert_eq!(Solution::max_frequency(vec![1, 4, 5], 1, 2), 2);
    assert_eq!(Solution::max_frequency(vec![5, 11, 20, 20], 5, 1), 2);
    assert_eq!(Solution::max_frequency(vec![2, 33], 47, 0), 1);
}

struct Solution;
impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        if num_operations > 0 {
            return Self::max_frequency(nums, k + num_operations, 0);
        }
        let n = nums.len();
        let mut nums = nums;
        nums.sort_unstable();

        let mut left = 0;
        let mut total = 0;
        let mut max_freq = 1;

        for right in 0..n {
            total += nums[right];

            while (nums[right] * (right - left + 1) as i32) - total > k {
                total -= nums[left];
                left += 1;
            }

            max_freq = max_freq.max((right - left + 1) as i32);
        }

        max_freq
    }
}
