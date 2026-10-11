fn main() {
    assert_eq!(
        Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]]),
        2
    );
    assert_eq!(
        Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]]),
        4
    );
    assert_eq!(Solution::unique_paths_iii(vec![vec![0, 1], vec![2, 0]]), 0);
}

struct Solution;
impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut total = 0usize;

        for i in 0..m {
            for j in 0..n {
                match grid[i][j] {
                    1 => {
                        start = (i, j);
                        total += 1;
                    }
                    2 => {
                        end = (i, j);
                        total += 1;
                    }
                    0 => total += 1,
                    _ => {}
                }
            }
        }

        let mut visited = vec![vec![false; n]; m];
        visited[start.0][start.1] = true;
        let mut ans = 0;
        Self::dfs(&grid, &mut visited, start.0, start.1, end, total, 1, &mut ans);
        ans
    }

    fn dfs(
        grid: &Vec<Vec<i32>>,
        visited: &mut Vec<Vec<bool>>,
        r: usize,
        c: usize,
        end: (usize, usize),
        total: usize,
        visited_count: usize,
        ans: &mut i32,
    ) {
        if (r, c) == end {
            if visited_count == total {
                *ans += 1;
            }
            return;
        }

        let directions = [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)];
        for (dr, dc) in directions {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nc < 0 {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            if nr >= grid.len() || nc >= grid[0].len() {
                continue;
            }
            if visited[nr][nc] || grid[nr][nc] == -1 {
                continue;
            }

            visited[nr][nc] = true;
            Self::dfs(
                grid,
                visited,
                nr,
                nc,
                end,
                total,
                visited_count + 1,
                ans,
            );
            visited[nr][nc] = false;
        }
    }
}
