fn main() {
    assert_eq!(
        Solution::closed_island(vec![
            vec![1, 1, 1, 1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 1, 0],
            vec![1, 0, 1, 0, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 0]
        ]),
        2
    );
    assert_eq!(
        Solution::closed_island(vec![vec![0, 0, 1, 0, 0], vec![0, 1, 0, 1, 0], vec![0, 1, 1, 1, 0]]),
        1
    );
    assert_eq!(
        Solution::closed_island(vec![
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1]
        ]),
        2
    );
}

struct Solution;
impl Solution {
    fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> bool {
        if i == 0 || j == 0 || i == grid.len() - 1 || j == grid[0].len() - 1 {
            return false;
        }
        if grid[i][j] == 1 {
            return true;
        }
        grid[i][j] = 1;
        let mut res = true;
        if !Self::dfs(grid, i + 1, j) {
            res = false;
        }
        if !Self::dfs(grid, i - 1, j) {
            res = false;
        }
        if !Self::dfs(grid, i, j + 1) {
            res = false;
        }
        if !Self::dfs(grid, i, j - 1) {
            res = false;
        }
        return res;
    }
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 {
                    if Self::dfs(&mut grid, i, j) {
                        count += 1;
                    }
                }
            }
        }
        return count;
    }
}
