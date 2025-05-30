fn main() {
    assert_eq!(Solution::subset_xor_sum(vec![1, 3]), 6);
    assert_eq!(Solution::subset_xor_sum(vec![5, 1, 6]), 28);
    assert_eq!(Solution::subset_xor_sum(vec![3, 4, 5, 6, 7, 8]), 480);
}

struct Solution;
impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        return nums.iter().fold(0, |acc, &num| acc ^ num) * (1 << (nums.len() - 1));
    }
}
