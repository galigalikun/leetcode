fn main() {
    assert_eq!(Solution::grid_game(vec![vec![2, 5, 4], vec![1, 5, 1]]), 4);
    assert_eq!(Solution::grid_game(vec![vec![3, 3, 1], vec![8, 5, 2]]), 4);
    assert_eq!(
        Solution::grid_game(vec![vec![1, 3, 1, 15], vec![1, 3, 3, 1]]),
        7
    );
}

struct Solution;
impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid[0].len();
        let mut top_sum = vec![0; n + 1];
        let mut bottom_sum = vec![0; n + 1];
        for i in 0..n {
            top_sum[i + 1] = top_sum[i] + grid[0][i];
            bottom_sum[i + 1] = bottom_sum[i] + grid[1][i];
        }
        let mut res = i64::MAX;
        for i in 0..n {
            res = res.min(top_sum[i] as i64 + bottom_sum[n] as i64 - bottom_sum[i + 1] as i64);
        }
        res
    }
}
