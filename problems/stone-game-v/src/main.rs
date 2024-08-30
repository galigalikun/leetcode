fn main() {
    assert_eq!(Solution::stone_game_v(vec![6, 2, 3, 4, 5, 5]), 18);
    assert_eq!(Solution::stone_game_v(vec![7, 7, 7, 7, 7, 7, 7]), 28);
    assert_eq!(Solution::stone_game_v(vec![4]), 0);
}

struct Solution;
impl Solution {
    pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        let n = stone_value.len();
        let mut dp = vec![vec![0; n]; n];
        let mut sum = vec![0; n + 1];
        for v in 0..n {
            sum[v + 1] = sum[v] + stone_value[v];
        }
        for len in 2..=n {
            for i in 0..n - len + 1 {
                let j = i + len - 1;
                for k in i..j {
                    let l = sum[k + 1] - sum[i];
                    let r = sum[j + 1] - sum[k + 1];
                    if l < r {
                        dp[i][j] = dp[i][j].max(l + dp[i][k]);
                    } else if l > r {
                        dp[i][j] = dp[i][j].max(r + dp[k + 1][j]);
                    } else {
                        dp[i][j] = dp[i][j].max(l + dp[i][k].max(dp[k + 1][j]));
                    }
                }
            }
        }
        dp[0][n - 1]
    }
}
