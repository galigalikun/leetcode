fn main() {
    assert_eq!(
        Solution::find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7]),
        3
    );
    assert_eq!(
        Solution::find_length(vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0]),
        5
    );
    assert_eq!(
        Solution::find_length(vec![0, 1, 1, 1, 1], vec![1, 0, 1, 0, 1]),
        2
    );
}

struct Solution {}
impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];
        for i in 1..=nums1.len() {
            for j in 1..=nums2.len() {
                if nums1[i - 1] == nums2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
                }
            }
        }
        return dp[nums1.len()][nums2.len()];
    }
}
