fn main() {
    assert_eq!(Solution::number_of_stable_arrays(1, 1, 2), 2);
    assert_eq!(Solution::number_of_stable_arrays(1, 2, 1), 1);
    assert_eq!(Solution::number_of_stable_arrays(3, 3, 2), 14);
}

struct Solution;
impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let mod_num = 1_000_000_007;
        let (z, o) = (zero as i64, one as i64);
        let l = limit as i64;
        let mut dp = vec![vec![0i64; (o + 1) as usize]; (z + 1) as usize];
        dp[0][0] = 1;
        for i in 0..=z {
            for j in 0..=o {
                if i == 0 && j == 0 {
                    continue;
                }
                let mut val = 0i64;
                for k in 1..=l {
                    if i >= k {
                        val = (val + dp[(i - k) as usize][j as usize]) % mod_num;
                    }
                    if j >= k {
                        val = (val + dp[i as usize][(j - k) as usize]) % mod_num;
                    }
                }
                dp[i as usize][j as usize] = val;
            }
        }
        return dp[z as usize][o as usize] as i32;
    }
}
