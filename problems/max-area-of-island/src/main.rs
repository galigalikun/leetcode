fn main() {
    assert_eq!(
        Solution::max_area_of_island(vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
        ]),
        6
    );
    assert_eq!(
        Solution::max_area_of_island(vec![vec![0, 0, 0, 0, 0, 0, 0, 0]]),
        0
    );
}

struct Solution {}
impl Solution {
    fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        if i >= grid.len() || j >= grid[0].len() || grid[i][j] == 0 {
            return 0;
        }
        grid[i][j] = 0;
        let mut area = 1;
        area += Solution::dfs(grid, i + 1, j);
        if i > 0 {
            area += Solution::dfs(grid, i - 1, j);
        }
        area += Solution::dfs(grid, i, j + 1);
        if j > 0 {
            area += Solution::dfs(grid, i, j - 1);
        }
        area
    }
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut max_area = 0;
        let mut grid = grid;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    let area = Self::dfs(&mut grid, i, j);
                    max_area = std::cmp::max(max_area, area);
                }
            }
        }
        return max_area;
    }
}
