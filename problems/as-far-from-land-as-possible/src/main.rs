fn main() {
    assert_eq!(
        Solution::max_distance(vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]),
        2
    );
    assert_eq!(
        Solution::max_distance(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]),
        4
    );
}

struct Solution;
impl Solution {
    fn bfs(grid: &mut Vec<Vec<i32>>, queue: &mut std::collections::VecDeque<(i32, i32)>) -> i32 {
        let mut count = 0;
        while queue.len() > 0 {
            let mut size = queue.len();
            while size > 0 {
                let (i, j) = queue.pop_front().unwrap();
                if i > 0 && grid[i as usize - 1][j as usize] == 0 {
                    grid[i as usize - 1][j as usize] = 1;
                    queue.push_back((i - 1, j));
                }
                if i < grid.len() as i32 - 1 && grid[i as usize + 1][j as usize] == 0 {
                    grid[i as usize + 1][j as usize] = 1;
                    queue.push_back((i + 1, j));
                }
                if j > 0 && grid[i as usize][j as usize - 1] == 0 {
                    grid[i as usize][j as usize - 1] = 1;
                    queue.push_back((i, j - 1));
                }
                if j < grid[0].len() as i32 - 1 && grid[i as usize][j as usize + 1] == 0 {
                    grid[i as usize][j as usize + 1] = 1;
                    queue.push_back((i, j + 1));
                }
                size -= 1;
            }
            count += 1;
        }
        return count - 1;
    }
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let mut queue = std::collections::VecDeque::new();
        let mut grid = grid;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 1 {
                    queue.push_back((i as i32, j as i32));
                }
            }
        }
        if queue.len() == 0 || queue.len() == grid.len() * grid[0].len() {
            return -1;
        }
        return Self::bfs(&mut grid, &mut queue);
    }
}
