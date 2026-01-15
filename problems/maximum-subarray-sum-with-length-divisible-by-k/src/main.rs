fn main() {
    assert_eq!(Solution::max_subarray_sum(vec![1, 2], 1), 3);
    assert_eq!(Solution::max_subarray_sum(vec![-1, -2, -3, -4, -5], 4), -10);
    assert_eq!(Solution::max_subarray_sum(vec![-5, 1, 2, -3, 4], 2), 4);
}

struct Solution;
impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;
        if k > n {
            return 0;
        }
        let mut max_sum = i64::MIN;
        let mut current_sum: i64 = nums.iter().take(k).map(|&x| x as i64).sum();
        max_sum = max_sum.max(current_sum);
        for i in k..n {
            current_sum += nums[i] as i64 - nums[i - k] as i64;
            max_sum = max_sum.max(current_sum);
        }
        return max_sum;
    }
}
