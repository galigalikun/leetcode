fn main() {
    assert_eq!(Solution::number_of_sets(4, 2), 5);
    assert_eq!(Solution::number_of_sets(3, 1), 3);
    assert_eq!(Solution::number_of_sets(30, 7), 796297179);
}

struct Solution;
impl Solution {
    pub fn number_of_sets(n: i32, k: i32) -> i32 {
        let n = n as i64;
        let k = k as i64;
        let mut dp = vec![vec![0; n as usize]; k as usize];
        let mut sum = vec![vec![0; n as usize]; k as usize];
        let r#mod = 1_000_000_007;
        for i in 0..n {
            dp[0][i as usize] = i * (n - i);
            sum[0][i as usize] = dp[0][i as usize];
            if i > 0 {
                sum[0][i as usize] += sum[0][(i - 1) as usize];
                sum[0][i as usize] %= r#mod;
            }
        }
        for i in 1..k {
            for j in 0..n {
                dp[i as usize][j as usize] = sum[(i - 1) as usize][j as usize];
                if j > 0 {
                    dp[i as usize][j as usize] += dp[i as usize][(j - 1) as usize];
                    dp[i as usize][j as usize] %= r#mod;
                }
                sum[i as usize][j as usize] = dp[i as usize][j as usize];
                if j > 0 {
                    sum[i as usize][j as usize] += sum[i as usize][(j - 1) as usize];
                    sum[i as usize][j as usize] %= r#mod;
                }
            }
        }
        dp[(k - 1) as usize][(n - 1) as usize] as i32
    }
}
