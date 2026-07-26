fn main() {
    assert_eq!(Solution::max_increase_keeping_skyline(vec![vec![3,0,8,4],vec![2,4,5,7],vec![9,2,6,3],vec![0,3,1,0]]), 35);
    assert_eq!(Solution::max_increase_keeping_skyline(vec![vec![0,0,0],vec![0,0,0],vec![0,0,0]]), 0);
}

struct Solution{}
impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        if n == 0 {
            return 0;
        }

        let row_maxes: Vec<i32> = grid
            .iter()
            .map(|row| row.iter().copied().max().unwrap_or(0))
            .collect();

        let col_maxes: Vec<i32> = (0..n)
            .map(|col| grid.iter().map(|row| row[col]).max().unwrap_or(0))
            .collect();

        let mut total_increase = 0;
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, &height) in row.iter().enumerate() {
                let allowed_height = row_maxes[row_idx].min(col_maxes[col_idx]);
                total_increase += allowed_height - height;
            }
        }

        total_increase
    }
}
