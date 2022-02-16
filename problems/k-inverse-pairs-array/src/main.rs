fn main() {
    assert_eq!(Solution::k_inverse_pairs(3, 0), 1);
    assert_eq!(Solution::k_inverse_pairs(3, 1), 2);
    assert_eq!(Solution::k_inverse_pairs(3, 3), 1);
    assert_eq!(Solution::k_inverse_pairs(1000, 1000), 663677020);
}

// https://dreamume.medium.com/leetcode-629-k-inverse-pairs-array-8f6b1c05e3ea
struct Solution {}
impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        let a = 1000000007;
        if k > n * (n - 1) / 2 || k < 0 {
            return 0;
        }
        let mut dp = vec![vec![0 as i64; k as usize + 1]; n as usize + 1];
        for i in 1..=n as usize {
            dp[i][0] = 1;
            if i + 1 <= n as usize {
                dp[i + 1][0] = 1;
            }
            for j in 1..=std::cmp::min(k as usize, i * (i - 1) / 2) {
                dp[i][j] = dp[i][j - 1] + dp[i - 1][j];
                if j >= i {
                    dp[i][j] -= dp[i - 1][j - i];
                }
                dp[i][j] = (dp[i][j] + a) % a;
            }
        }
        return dp[n as usize][k as usize] as i32;
    }
}
