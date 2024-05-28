fn main() {
    assert_eq!(
        Solution::max_dot_product(vec![2, 1, -2, 5], vec![3, 0, -6]),
        18
    );
    assert_eq!(Solution::max_dot_product(vec![3, -2], vec![2, -6, 7]), 21);
    assert_eq!(Solution::max_dot_product(vec![-1, -1], vec![1, 1]), -1);
}

struct Solution;
impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; nums2.len()]; nums1.len()];
        dp[0][0] = nums1[0] * nums2[0];
        for i in 1..nums1.len() {
            dp[i][0] = std::cmp::max(dp[i - 1][0], nums1[i] * nums2[0]);
        }
        for j in 1..nums2.len() {
            dp[0][j] = std::cmp::max(dp[0][j - 1], nums1[0] * nums2[j]);
        }
        for i in 1..nums1.len() {
            for j in 1..nums2.len() {
                dp[i][j] = std::cmp::max(
                    std::cmp::max(dp[i - 1][j], dp[i][j - 1]),
                    std::cmp::max(dp[i - 1][j - 1], 0) + nums1[i] * nums2[j],
                );
            }
        }
        return dp[nums1.len() - 1][nums2.len() - 1];
    }
}
