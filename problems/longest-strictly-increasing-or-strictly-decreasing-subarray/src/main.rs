fn main() {
    assert_eq!(Solution::longest_monotonic_subarray(vec![1, 4, 3, 3, 2]), 2);
    assert_eq!(Solution::longest_monotonic_subarray(vec![3, 3, 3, 3]), 1);
    assert_eq!(Solution::longest_monotonic_subarray(vec![3, 2, 1]), 3);
}

struct Solution;
impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        let mut max_len = 1;
        let mut inc_len = 1;
        let mut dec_len = 1;
        for i in 1..n {
            if nums[i] >= nums[i - 1] {
                inc_len += 1;
            } else {
                inc_len = 1;
            }
            if nums[i] <= nums[i - 1] {
                dec_len += 1;
            } else {
                dec_len = 1;
            }
            max_len = max_len.max(inc_len).max(dec_len);
        }
        max_len as i32
    }
}
