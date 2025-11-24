fn main() {
    assert_eq!(Solution::number_of_stable_arrays(1, 1, 2), 2);
    assert_eq!(Solution::number_of_stable_arrays(1, 2, 1), 1);
    assert_eq!(Solution::number_of_stable_arrays(3, 3, 2), 14);
}

struct Solution;
impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let mut dp = vec![vec![vec![0; 2]; (one + 1) as usize]; (zero + 1) as usize];
        let modulo = 1_000_000_007;
        dp[0][0][0] = 1;
        dp[0][0][1] = 1;
        for z in 0..=zero as usize {
            for o in 0..=one as usize {
                for last in 0..2 {
                    if last == 0 {
                        if z > 0 {
                            dp[z][o][0] = (dp[z][o][0] + dp[z - 1][o][0]) % modulo;
                        }
                        if limit > 1 && z > 0 {
                            dp[z][o][0] = (dp[z][o][0] + dp[z - 1][o][1]) % modulo;
                        }
                    } else {
                        if o > 0 {
                            dp[z][o][1] = (dp[z][o][1] + dp[z][o - 1][1]) % modulo;
                        }
                        if limit > 1 && o > 0 {
                            dp[z][o][1] = (dp[z][o][1] + dp[z][o - 1][0]) % modulo;
                        }
                    }
                }
            }
        }
        return (dp[zero as usize][one as usize][0] + dp[zero as usize][one as usize][1]) % modulo;
    }
}
