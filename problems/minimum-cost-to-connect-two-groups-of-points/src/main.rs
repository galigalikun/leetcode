fn main() {
    assert_eq!(
        Solution::connect_two_groups(vec![vec![15, 96], vec![36, 2]]),
        17
    );
    assert_eq!(
        Solution::connect_two_groups(vec![vec![1, 3, 5], vec![4, 1, 1], vec![1, 5, 3]]),
        4
    );
    assert_eq!(
        Solution::connect_two_groups(vec![
            vec![2, 5, 1],
            vec![3, 4, 7],
            vec![8, 1, 2],
            vec![6, 2, 4],
            vec![3, 8, 8]
        ]),
        10
    );
}

struct Solution;
impl Solution {
    pub fn connect_two_groups(cost: Vec<Vec<i32>>) -> i32 {
        let n = cost.len();
        let m = cost[0].len();
        let mut dp = vec![vec![std::i32::MAX; 1 << m]; n + 1];
        dp[0][0] = 0;
        for i in 0..n {
            for j in 0..1 << m {
                for k in 0..m {
                    if dp[i][j] != std::i32::MAX {
                        dp[i + 1][j | 1 << k] = dp[i + 1][j | 1 << k].min(dp[i][j] + cost[i][k]);
                        dp[i + 1][j] = dp[i + 1][j].min(dp[i][j] + cost[i][k]);
                    }
                }
            }
        }
        let mut ans = std::i128::MAX;
        for i in 0..1 << m {
            for j in 0..n {
                ans = ans.min(dp[j][i] as i128 + dp[n][i] as i128);
            }
        }
        ans as i32
    }
}
