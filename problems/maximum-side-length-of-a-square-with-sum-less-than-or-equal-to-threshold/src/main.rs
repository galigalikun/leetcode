fn main() {
    assert_eq!(
        Solution::max_side_length(
            vec![
                vec![1, 1, 3, 2, 4, 3, 2],
                vec![1, 1, 3, 2, 4, 3, 2],
                vec![1, 1, 3, 2, 4, 3, 2]
            ],
            4
        ),
        2
    );
    assert_eq!(
        Solution::max_side_length(
            vec![
                vec![2, 2, 2, 2, 2],
                vec![2, 2, 2, 2, 2],
                vec![2, 2, 2, 2, 2],
                vec![2, 2, 2, 2, 2],
                vec![2, 2, 2, 2, 2]
            ],
            1
        ),
        0
    );
}

struct Solution;
impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let mut dp = vec![vec![0; mat[0].len() + 1]; mat.len() + 1];
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                dp[i + 1][j + 1] = dp[i + 1][j] + dp[i][j + 1] - dp[i][j] + mat[i][j];
            }
        }
        let mut ans = 0;
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                let l = ans;
                let r = std::cmp::min(mat.len() - i, mat[0].len() - j);
                while l < r {
                    let mid = (l + r + 1) / 2;
                    let sum = dp[i + mid][j + mid] - dp[i + mid][j] - dp[i][j + mid] + dp[i][j];
                    ans = if sum <= threshold { mid } else { ans };
                }
            }
        }
        return ans as i32;
    }
}
