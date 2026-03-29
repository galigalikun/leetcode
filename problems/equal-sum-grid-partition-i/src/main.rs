fn main() {
    assert_eq!(
        Solution::can_partition_grid(vec![[1, 4], [2, 3]].iter().map(|&x| x.to_vec()).collect()),
        true
    );
    assert_eq!(
        Solution::can_partition_grid(vec![[1, 3], [2, 4]].iter().map(|&x| x.to_vec()).collect()),
        false
    );
    assert_eq!(
        Solution::can_partition_grid(
            vec![
                [88815, 75184, 58058, 38864, 87921],
                [97879, 72561, 30833, 11376, 11509],
                [59407, 20659, 8972, 7527, 45639],
                [74290, 33652, 45742, 34852, 37622],
                [6582, 37220, 5854, 79095, 71801],
                [82868, 19595, 5356, 381, 60132],
                [48089, 85386, 87274, 51062, 39949]
            ]
            .iter()
            .map(|&x| x.to_vec())
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

        let total_sum: i32 = row_sum.iter().sum();
        if total_sum % 2 != 0 {
            return false;
        }

        let half_sum = total_sum / 2;
        let mut dp = vec![false; (half_sum + 1) as usize];
        dp[0] = true;

        for &num in &row_sum {
            for j in (num..=half_sum).rev() {
                dp[j as usize] |= dp[(j - num) as usize];
            }
        }

        for &num in &col_sum {
            for j in (num..=half_sum).rev() {
                dp[j as usize] |= dp[(j - num) as usize];
            }
        }

        dp[half_sum as usize]
    }
}
