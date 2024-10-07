fn main() {
    assert_eq!(
        Solution::restore_matrix(vec![3, 8], vec![4, 7]),
        vec![vec![3, 0], vec![1, 7]]
    );
    assert_eq!(
        Solution::restore_matrix(vec![5, 7, 10], vec![8, 6, 8]),
        vec![[0, 5, 0], [6, 1, 0], [2, 0, 8]]
    )
}

struct Solution;
impl Solution {
    pub fn restore_matrix(row_sum: Vec<i32>, col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut row_sum = row_sum;
        let mut col_sum = col_sum;
        let mut res = vec![vec![0; col_sum.len()]; row_sum.len()];
        for i in 0..row_sum.len() {
            for j in 0..col_sum.len() {
                res[i][j] = row_sum[i].min(col_sum[j]);
                row_sum[i] -= res[i][j];
                col_sum[j] -= res[i][j];
            }
        }
        res
    }
}
