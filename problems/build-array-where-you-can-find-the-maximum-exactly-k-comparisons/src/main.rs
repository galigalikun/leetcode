fn main() {
    assert_eq!(Solution::num_of_arrays(2, 3, 1), 6);
    assert_eq!(Solution::num_of_arrays(5, 2, 3), 0);
    assert_eq!(Solution::num_of_arrays(9, 1, 1), 1);
}

struct Solution;
impl Solution {
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        let mut dp = vec![vec![vec![0; m as usize + 1]; k as usize + 1]; n as usize + 1];
        let r#mod = 1_000_000_007;
        for i in 1..=m as usize {
            dp[1][1][i] = 1;
        }
        for i in 2..=n as usize {
            for j in 1..=k as usize {
                for l in 1..=m as usize {
                    for p in 1..=m as usize {
                        if p < l {
                            dp[i][j][l] += dp[i - 1][j - 1][p];
                            dp[i][j][l] %= r#mod;
                        }
                    }
                }
            }
        }
        let mut ans = 0;
        for i in 1..=m as usize {
            ans += dp[n as usize][k as usize][i];
            ans %= r#mod;
        }
        return ans;
    }
}
