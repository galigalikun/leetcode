fn main() {
    assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);

    assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);

    assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
}

struct Solution {}
// https://www.geeksforgeeks.org/longest-increasing-subsequence-dp-3/
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len()];

        for i in 1..nums.len() {
            for j in 0..i {
                if nums[i] > nums[j] && dp[i] < dp[j] + 1 {
                    dp[i] = dp[j] + 1;
                }
            }
        }

        return dp.into_iter().max().unwrap();
    }
}
