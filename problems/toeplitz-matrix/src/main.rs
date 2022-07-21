fn main() {
    assert_eq!(
        Solution::is_toeplitz_matrix(vec![vec![1, 2, 3, 4], vec![5, 1, 2, 3], vec![9, 5, 1, 2]]),
        true
    );
    assert_eq!(
        Solution::is_toeplitz_matrix(vec![vec![1, 2], vec![2, 2]]),
        false
    );
}

struct Solution {}
impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        for i in 1..matrix.len() {
            for j in 1..matrix[i].len() {
                if matrix[i][j] != matrix[i - 1][j - 1] {
                    return false;
                }
            }
        }
        return true;
    }
}
