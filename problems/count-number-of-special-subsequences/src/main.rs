fn main() {
    assert_eq!(Solution::count_special_subsequences(vec![0, 1, 2, 2]), 3);
    assert_eq!(Solution::count_special_subsequences(vec![2, 2, 0, 0]), 0);
    assert_eq!(
        Solution::count_special_subsequences(vec![0, 1, 2, 0, 1, 2]),
        7
    );
}

struct Solution;
impl Solution {
    pub fn count_special_subsequences(nums: Vec<i32>) -> i32 {
        let modulo = 1_000_000_007;
        let n = nums.len();
        let mut dp = vec![0; n + 1];
        dp[0] = 1;

        for i in 1..=n {
            dp[i] = (dp[i - 1] * 2) % modulo;
            if nums[i - 1] == 0 {
                dp[i] = (dp[i] - dp[i - 1]) % modulo;
            }
        }

        return dp[n];
    }
}
