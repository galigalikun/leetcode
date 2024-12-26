fn main() {
    assert_eq!(Solution::stone_game_vii(vec![5, 3, 1, 4, 2]), 6);
    assert_eq!(
        Solution::stone_game_vii(vec![7, 90, 5, 1, 100, 10, 10, 2]),
        122
    );
}

struct Solution;
impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut dp = vec![vec![0; n]; n];
        for i in stones.iter().enumerate() {
            dp[i.0][i.0] = *i.1;
        }
        for i in (0..n).rev() {
            for j in i + 1..n {
                dp[i][j] = std::cmp::max(stones[i] - dp[i + 1][j], stones[j] - dp[i][j - 1]);
            }
        }
        dp[0][n - 1]
    }
}
