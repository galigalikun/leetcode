fn main() {
    assert_eq!(
        Solution::max_uncrossed_lines(vec![1, 4, 2], vec![1, 2, 4]),
        2
    );
    assert_eq!(
        Solution::max_uncrossed_lines(vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2]),
        3
    );
    assert_eq!(
        Solution::max_uncrossed_lines(vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1]),
        2
    );
}

struct Solution;
impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];
        for i in 1..=nums1.len() {
            for j in 1..=nums2.len() {
                if nums1[i - 1] == nums2[j - 1] {
                    dp[i][j] = std::cmp::max(dp[i][j], dp[i - 1][j - 1] + 1);
                } else {
                    dp[i][j] = std::cmp::max(dp[i][j - 1], dp[i - 1][j]);
                }
            }
        }
        return dp[nums1.len()][nums2.len()];
    }
}
