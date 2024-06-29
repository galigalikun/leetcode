fn main() {
    assert_eq!(Solution::longest_subarray(vec![1, 1, 0, 1]), 3);
    assert_eq!(
        Solution::longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]),
        5
    );
    assert_eq!(Solution::longest_subarray(vec![1, 1, 1]), 2);
}

struct Solution;
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut zero_count = 0;
        let mut max_len = 0;
        for right in 0..nums.len() {
            if nums[right] == 0 {
                zero_count += 1;
            }
            while zero_count > 1 {
                if nums[left] == 0 {
                    zero_count -= 1;
                }
                left += 1;
            }
            max_len = max_len.max(right - left);
        }
        if max_len == nums.len() {
            return max_len as i32 - 1;
        }
        return max_len as i32;
    }
}
