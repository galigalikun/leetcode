fn main() {
    assert_eq!(Solution::max_frequency(vec![1, 2, 4], 5), 3);
    assert_eq!(Solution::max_frequency(vec![1, 4, 8, 13], 5), 2);
    assert_eq!(Solution::max_frequency(vec![3, 9, 6], 2), 1);
}

struct Solution;
impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut left = 0;
        let mut right = 0;
        let mut sum = 0;
        let mut max_freq = 0;
        let n = nums.len();
        while right < n {
            sum += nums[right];
            while nums[right] * (right as i32 - left as i32 + 1) - sum > k {
                sum -= nums[left];
                left += 1;
            }
            max_freq = max_freq.max(right - left + 1);
            right += 1;
        }
        max_freq as i32
    }
}
