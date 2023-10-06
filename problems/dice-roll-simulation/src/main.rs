fn main() {
    assert_eq!(Solution::die_simulator(2, vec![1, 1, 2, 2, 2, 3]), 34);
    assert_eq!(Solution::die_simulator(2, vec![1, 1, 1, 1, 1, 1]), 30);
    assert_eq!(Solution::die_simulator(3, vec![1, 1, 1, 2, 2, 3]), 181);
}

struct Solution;
impl Solution {
    pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
        let mut dp = vec![vec![vec![0; 16]; 7]; n as usize + 1];
        for i in 1..=6 {
            dp[1][i][1] = 1;
        }
        for i in 2..=n as usize {
            for j in 1..=6 as usize {
                for k in 1..=6 as usize {
                    if j == k {
                        for l in 1..=roll_max[k as usize - 1] as usize {
                            if l != j {
                                dp[i][j][1] += dp[i - 1][k][l];
                                dp[i][j][1] %= 1000000007;
                            }
                        }
                    } else {
                        for l in 1..=roll_max[k as usize - 1] as usize {
                            dp[i][j][1] += dp[i - 1][k][l];
                            dp[i][j][1] %= 1000000007;
                        }
                    }
                }
            }
            for j in 1..=6 {
                for k in 1..=roll_max[j as usize - 1] as usize {
                    dp[i][j][k as usize] = dp[i - 1][j][k - 1];
                }
            }
        }
        let mut ans = 0;
        for i in 1..=6 {
            for j in 1..=roll_max[i as usize - 1] as usize {
                ans += dp[n as usize][i][j];
                ans %= 1000000007;
            }
        }
        return ans;
    }
}
