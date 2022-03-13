fn main() {
    assert_eq!(
        Solution::maximal_square(vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0']
        ]),
        4
    );

    assert_eq!(
        Solution::maximal_square(vec![vec!['0', '1'], vec!['1', '0']]),
        1
    );

    assert_eq!(Solution::maximal_square(vec![vec!['0']]), 0);
}

struct Solution {}
// https://afteracademy.com/blog/maximal-square
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut max_square = 0;
        let mut dp = vec![vec![0; cols + 1]; rows + 1];
        for i in 1..=rows {
            for j in 1..=cols {
                if matrix[i - 1][j - 1] == '1' {
                    dp[i][j] =
                        std::cmp::min(std::cmp::min(dp[i - 1][j], dp[i][j - 1]), dp[i - 1][j - 1])
                            + 1;
                }
                if dp[i][j] > max_square {
                    max_square = dp[i][j];
                }
            }
        }

        return max_square * max_square;
    }
}
