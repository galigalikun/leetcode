fn main() {
    assert_eq!(Solution::num_enclaves(vec![vec![0,0,0,0],vec![1,0,1,0],vec![0,1,1,0],vec![0,0,0,0]]), 3);
    assert_eq!(Solution::num_enclaves(vec![vec![0,1,1,0],vec![0,0,1,0],vec![0,0,1,0],vec![0,0,0,0]]), 0);
}

struct Solution;
impl Solution {
    fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        println!("dfs {} {} {:?}", i, j, grid);
        if i >= grid.len() || j >= grid[0].len() {
            println!("return2 0");
            return 0;
        }
        if grid[i][j] == 0 {
            println!("return3 0");
            return 0;
        }
        grid[i][j] = 0;
        println!("debug {} {}", i, j);
        return 1 + Self::dfs(grid, i + 1, j) + Self::dfs(grid, i - 1, j) + Self::dfs(grid, i, j + 1) + Self::dfs(grid, i, j - 1);
    }
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    println!("count {}, {}", i, j);
                    count += Self::dfs(&mut grid, i, j);
                }
            }
        }
        return count;
    }
}
