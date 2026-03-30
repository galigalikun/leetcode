fn main() {
    assert_eq!(
        Solution::can_partition_grid(
            vec![[1, 4], [2, 3]]
                .iter()
                .map(|&row| row.to_vec())
                .collect()
        ),
        true
    );
    assert_eq!(
        Solution::can_partition_grid(
            vec![[1, 2], [3, 4]]
                .iter()
                .map(|&row| row.to_vec())
                .collect()
        ),
        true
    );
    assert_eq!(
        Solution::can_partition_grid(
            vec![[1, 2, 4], [2, 3, 5]]
                .iter()
                .map(|&row| row.to_vec())
                .collect()
        ),
        false
    );
}

struct Solution;
impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let mut row_sum = vec![0; m];
        let mut col_sum = vec![0; n];
        for i in 0..m {
            for j in 0..n {
                row_sum[i] += grid[i][j];
                col_sum[j] += grid[i][j];
            }
        }
        row_sum.sort_unstable();
        col_sum.sort_unstable();
        row_sum == col_sum
    }
}
