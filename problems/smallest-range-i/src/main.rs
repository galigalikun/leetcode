fn main() {
    assert_eq!(Solution::smallest_range_i(vec![1], 0), 0);
    assert_eq!(Solution::smallest_range_i(vec![0, 10], 2), 6);
    assert_eq!(Solution::smallest_range_i(vec![1, 3, 6], 3), 0);
}

struct Solution;
impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let max = nums.iter().copied().max().unwrap_or(0);
        let min = nums.iter().copied().min().unwrap_or(0);
        (max - min - 2 * k).max(0)
    }
}
