fn main() {
    assert_eq!(Solution::matrix_score(vec![vec![0,0,1,1],vec![1,0,1,0],vec![1,1,0,0]]), 39);
    assert_eq!(Solution::matrix_score(vec![vec![0]]), 1);
}

struct Solution;
impl Solution {
    pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
        let row_count = grid.len();
        let col_count = grid[0].len();

        let mut score = 0_i32;
        for col in 0..col_count {
            let ones = grid
                .iter()
                .filter(|row| {
                    let row_flipped = row[0] == 0;
                    let value = if row_flipped { 1 - row[col] } else { row[col] };
                    value == 1
                })
                .count();

            let max_ones = ones.max(row_count - ones) as i32;
            score += max_ones * (1_i32 << (col_count - 1 - col));
        }

        score
    }
}
