fn main() {
    assert_eq!(Solution::wiggle_max_length(vec![1, 7, 4, 9, 2, 5]), 6);
    assert_eq!(
        Solution::wiggle_max_length(vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8]),
        7
    );
    assert_eq!(
        Solution::wiggle_max_length(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
        2
    );
}

struct Solution {}
// https://levelup.gitconnected.com/march-leetcoding-challenge-2021-day-18-wiggle-subsequence-3c408287325b
impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut up = vec![1; nums.len()];
        let mut down = vec![1; nums.len()];

        for i in 1..nums.len() {
            for j in 0..i {
                if nums[i] > nums[j] {
                    up[i] = std::cmp::max(up[i], 1 + down[j]);
                } else if nums[i] < nums[j] {
                    down[i] = std::cmp::max(down[i], 1 + up[j]);
                }
            }
        }
        return std::cmp::max(up[nums.len() - 1], down[nums.len() - 1]);
    }
}
