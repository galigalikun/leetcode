pub fn main() {
    assert_eq!(
        Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
        7
    );
    assert_eq!(
        Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]),
        12
    );
    assert_eq!(Solution::min_path_sum(vec![vec![1, 2], vec![1, 1]]), 3);
    assert_eq!(
        Solution::min_path_sum(vec![vec![1, 2, 5], vec![3, 2, 1]]),
        6
    );
}

pub struct Solution {}
// https://www.programcreek.com/2014/05/leetcode-minimum-path-sum-java/
impl Solution {
    fn dfs(i: usize, j: usize, grid: Vec<Vec<i32>>) -> i32 {
        if i == grid.len() - 1 && j == grid[0].len() - 1 {
            return grid[i][j];
        }

        if i < grid.len() - 1 && j < grid[0].len() - 1 {
            let r1 = grid[i][j] + Solution::dfs(i + 1, j, grid.clone());
            let r2 = grid[i][j] + Solution::dfs(i, j + 1, grid);
            return std::cmp::min(r1, r2);
        }

        if i < grid.len() - 1 {
            return grid[i][j] + Solution::dfs(i + 1, j, grid);
        }

        return grid[i][j] + Solution::dfs(i, j + 1, grid);
    }
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        /*
        1,2,5
        3,2,1
        */
        return Solution::dfs(0, 0, grid);
    }
}
