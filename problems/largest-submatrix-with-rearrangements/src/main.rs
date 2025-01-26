fn main() {
    assert_eq!(
        Solution::largest_submatrix(vec![vec![0, 0, 1], vec![1, 1, 1], vec![1, 0, 1]]),
        4
    );
    assert_eq!(Solution::largest_submatrix(vec![vec![1, 0, 1, 0, 1]]), 3);
    assert_eq!(
        Solution::largest_submatrix(vec![vec![1, 1, 0], vec![1, 0, 1]]),
        2
    );
}

struct Solution;
impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let mut matrix = matrix;
        let n = matrix.len();
        let m = matrix[0].len();
        for i in 1..n {
            for j in 0..m {
                if matrix[i][j] == 1 {
                    matrix[i][j] += matrix[i - 1][j];
                }
            }
        }
        let mut res = 0;
        for i in 0..n {
            matrix[i].sort_unstable_by(|a, b| b.cmp(a));
            for j in 0..m {
                res = res.max(matrix[i][j] * (j as i32 + 1));
            }
        }
        res
    }
}
