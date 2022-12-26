fn main() {
    assert_eq!(Solution::smallest_range_i(vec![1], 0), 0);
    assert_eq!(Solution::smallest_range_i(vec![0,10], 2), 6);
    assert_eq!(Solution::smallest_range_i(vec![1,3,6], 3), 0);
}

struct Solution;
impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let mut max = std::i32::MIN;
        let mut min = std::i32::MAX;
        for i in 0..nums.len() {
            max = std::cmp::max(nums[i], max);
            min = std::cmp::max(nums[i], min);
        }
        return 0;
    }
}
