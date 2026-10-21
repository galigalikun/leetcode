fn main() {
    assert_eq!(Solution::num_enclaves(vec![vec![0, 0, 0, 0], vec![1, 0, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]]), 3);
    assert_eq!(Solution::num_enclaves(vec![vec![0, 1, 1, 0], vec![0, 0, 1, 0], vec![0, 0, 1, 0], vec![0, 0, 0, 0]]), 0);
    assert_eq!(Solution::num_enclaves(vec![]), 0);
    assert_eq!(Solution::num_enclaves(vec![vec![]]), 0);
}

struct Solution;
impl Solution {
    fn dfs(grid: &mut Vec<Vec<i32>>, rows: usize, cols: usize, r: usize, c: usize) {
        if r >= rows || c >= cols || grid[r][c] == 0 {
            return;
        }

        grid[r][c] = 0;

        if r > 0 {
            Self::dfs(grid, rows, cols, r - 1, c);
        }
        if r + 1 < rows {
            Self::dfs(grid, rows, cols, r + 1, c);
        }
        if c > 0 {
            Self::dfs(grid, rows, cols, r, c - 1);
        }
        if c + 1 < cols {
            Self::dfs(grid, rows, cols, r, c + 1);
        }
    }

    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }

        let mut grid = grid;
        let rows = grid.len();
        let cols = grid[0].len();

        for c in 0..cols {
            if grid[0][c] == 1 {
                Self::dfs(&mut grid, rows, cols, 0, c);
            }
            if grid[rows - 1][c] == 1 {
                Self::dfs(&mut grid, rows, cols, rows - 1, c);
            }
        }

        for r in 0..rows {
            if grid[r][0] == 1 {
                Self::dfs(&mut grid, rows, cols, r, 0);
            }
            if grid[r][cols - 1] == 1 {
                Self::dfs(&mut grid, rows, cols, r, cols - 1);
            }
        }

        let mut count = 0;
        for row in &grid {
            for &cell in row {
                if cell == 1 {
                    count += 1;
                }
            }
        }

        count
    }
}
