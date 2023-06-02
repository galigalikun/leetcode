fn main() {
    assert_eq!(Solution::min_score_triangulation(vec![1, 2, 3]), 6);
    assert_eq!(Solution::min_score_triangulation(vec![3, 7, 4, 5]), 144);
    assert_eq!(
        Solution::min_score_triangulation(vec![1, 3, 1, 4, 1, 5]),
        13
    );
}

struct Solution;
impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; values.len()]; values.len()];
        for i in (0..values.len() - 1).rev() {
            for j in i + 1..values.len() {
                dp[i][j] = std::i32::MAX;
                for k in i + 1..j {
                    dp[i][j] =
                        dp[i][j].min(dp[i][k] + dp[k][j] + values[i] * values[j] * values[k]);
                }
            }
        }
        return dp[0][values.len() - 1];
    }
}
