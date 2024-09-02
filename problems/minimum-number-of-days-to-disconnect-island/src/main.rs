fn main() {
    assert_eq!(
        Solution::min_days(vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]]),
        2
    );
    assert_eq!(Solution::min_days(vec![vec![1, 1]]), 2);
}

struct Solution;
impl Solution {
    fn dfs(grid: &mut Vec<Vec<i32>>) -> i32 {
        let mut grid = grid.clone();
        let mut stack = vec![];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    stack.push((i as i32, j as i32));
                    break;
                }
            }
            if !stack.is_empty() {
                break;
            }
        }
        let mut count = 0;
        while !stack.is_empty() {
            let mut new_stack = vec![];
            for (i, j) in stack {
                if i < 0 || i >= grid.len() as i32 || j < 0 || j >= grid[0].len() as i32 {
                    continue;
                }
                if grid[i as usize][j as usize] == 0 {
                    continue;
                }
                grid[i as usize][j as usize] = 0;
                count += 1;
                new_stack.push((i - 1, j));
                new_stack.push((i + 1, j));
                new_stack.push((i, j - 1));
                new_stack.push((i, j + 1));
            }
            stack = new_stack;
        }
        count
    }
    pub fn min_days(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    grid[i][j] = 0;
                    if Self::dfs(&mut grid) > 1 {
                        return 1;
                    }
                    grid[i][j] = 1;
                }
            }
        }
        2
    }
}
