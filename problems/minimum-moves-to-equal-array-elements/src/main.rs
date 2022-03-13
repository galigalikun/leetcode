fn main() {
    assert_eq!(Solution::min_moves(vec![1, 2, 3]), 3);
    assert_eq!(Solution::min_moves(vec![1, 1, 1]), 0);
}

struct Solution {}
// https://www.geeksforgeeks.org/minimum-number-increment-operations-make-array-elements-equal/
impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut min = std::i32::MAX;
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            min = std::cmp::min(min, nums[i]);
        }
        return sum - (nums.len() as i32 * min);
    }
}
