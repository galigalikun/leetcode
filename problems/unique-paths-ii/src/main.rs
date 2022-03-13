fn main() {
    assert_eq!(
        Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
        2
    );
    assert_eq!(
        Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]),
        1
    );
    assert_eq!(
        Solution::unique_paths_with_obstacles(vec![vec![1], vec![0]]),
        0
    );
}

struct Solution {}
// https://www.youtube.com/watch?v=favJnfIiPSQ
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid[0][0] == 1 {
            return 0;
        }
        let mut result = Vec::new();
        let mut skip_i = false;
        let mut skip_j = false;
        for i in 0..obstacle_grid.len() {
            result.push(vec![]);
            for j in 0..obstacle_grid[i].len() {
                if i == 0 {
                    if obstacle_grid[i][j] == 0 && !skip_i {
                        result[i].push(1);
                    } else {
                        result[i].push(0);
                        skip_i = true;
                    }
                } else if j == 0 {
                    if obstacle_grid[i][j] == 0 && !skip_j {
                        result[i].push(1);
                    } else {
                        result[i].push(0);
                        skip_j = true;
                    }
                } else {
                    if obstacle_grid[i][j] == 1 {
                        result[i].push(0);
                    } else {
                        let diff = result[i][j - 1] + result[i - 1][j];
                        result[i].push(diff);
                    }
                }
            }
        }
        return result[obstacle_grid.len() - 1][obstacle_grid[0].len() - 1];
    }
}
