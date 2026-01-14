fn main() {
    assert_eq!(
        Solution::number_of_paths(vec![vec![5, 2, 4], vec![3, 0, 5], vec![0, 7, 2]], 3),
        2
    );
    assert_eq!(Solution::number_of_paths(vec![vec![0, 0]], 5), 1);
    assert_eq!(
        Solution::number_of_paths(
            vec![vec![7, 3, 4, 9], vec![2, 3, 6, 2], vec![2, 3, 7, 0]],
            1
        ),
        10
    );
}

struct Solution;
impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let m = grid.len();
        let n = grid[0].len();
        let k_usize = k as usize;
        let mut dp = vec![vec![vec![0i64; k_usize]; n]; m];

        for i in 0..m {
            for j in 0..n {
                let val_mod = ((grid[i][j] % k + k) % k) as usize;
                if i == 0 && j == 0 {
                    dp[0][0][val_mod] = 1;
                    continue;
                }

                if i > 0 {
                    for r in 0..k_usize {
                        let cnt = dp[i - 1][j][r];
                        if cnt == 0 {
                            continue;
                        }
                        let new_r = (r + val_mod) % k_usize;
                        dp[i][j][new_r] = (dp[i][j][new_r] + cnt) % MOD;
                    }
                }

                if j > 0 {
                    for r in 0..k_usize {
                        let cnt = dp[i][j - 1][r];
                        if cnt == 0 {
                            continue;
                        }
                        let new_r = (r + val_mod) % k_usize;
                        dp[i][j][new_r] = (dp[i][j][new_r] + cnt) % MOD;
                    }
                }
            }
        }

        dp[m - 1][n - 1][0] as i32
    }
}
