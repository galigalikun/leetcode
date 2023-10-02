fn main() {
    assert_eq!(
        Solution::get_maximum_gold(vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]]),
        24
    );
    assert_eq!(
        Solution::get_maximum_gold(vec![
            vec![1, 0, 7],
            vec![2, 0, 6],
            vec![3, 4, 5],
            vec![0, 3, 0],
            vec![9, 0, 20]
        ]),
        28
    );
}

struct Solution;
impl Solution {
    fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        if i >= grid.len() || j >= grid[0].len() || grid[i][j] == 0 {
            return 0;
        }
        let mut max = 0;
        let val = grid[i][j];
        grid[i][j] = 0;
        max = std::cmp::max(max, Self::dfs(grid, i + 1, j));
        if i > 0 {
            max = std::cmp::max(max, Self::dfs(grid, i - 1, j));
        }
        max = std::cmp::max(max, Self::dfs(grid, i, j + 1));
        if j > 0 {
            max = std::cmp::max(max, Self::dfs(grid, i, j - 1));
        }
        grid[i][j] = val;
        return max + val;
    }
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut max = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                max = std::cmp::max(max, Self::dfs(&mut grid, i, j));
            }
        }
        return max;
    }
}
