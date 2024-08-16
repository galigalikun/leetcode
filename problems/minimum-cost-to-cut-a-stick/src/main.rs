fn main() {
    assert_eq!(Solution::min_cost(7, vec![1, 3, 4, 5]), 16);
    assert_eq!(Solution::min_cost(9, vec![5, 6, 1, 4, 2]), 22);
}

struct Solution;
impl Solution {
    pub fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
        let mut cuts = cuts;
        cuts.sort();
        let mut dp = vec![vec![0; cuts.len() + 2]; cuts.len() + 2];
        let mut len = vec![0; cuts.len() + 2];
        for i in 0..cuts.len() {
            len[i + 1] = cuts[i];
        }
        len[cuts.len() + 1] = n;
        for i in (0..cuts.len() + 1).rev() {
            for j in i + 2..cuts.len() + 2 {
                dp[i][j] = i32::MAX;
                for k in i + 1..j {
                    dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j] + len[j] - len[i]);
                }
            }
        }
        dp[0][cuts.len() + 1]
    }
}
