fn main() {
    assert_eq!(
        Solution::shortest_path_binary_matrix(vec![vec![0, 1], vec![1, 0]]),
        2
    );
    assert_eq!(
        Solution::shortest_path_binary_matrix(vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]]),
        4
    );
    assert_eq!(
        Solution::shortest_path_binary_matrix(vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]]),
        -1
    );
}

struct Solution;
impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let mut queue = std::collections::VecDeque::new();
        let mut visited = std::collections::HashSet::new();
        let mut step = 0;
        let n = grid.len() as i32;
        if grid[0][0] == 1 || grid[n as usize - 1][n as usize - 1] == 1 {
            return -1;
        }
        queue.push_back((0, 0));
        visited.insert((0, 0));
        while !queue.is_empty() {
            let len = queue.len();
            for _ in 0..len {
                let (x, y) = queue.pop_front().unwrap();
                if x == n - 1 && y == n - 1 {
                    return step + 1;
                }
                for i in -1..=1 {
                    for j in -1..=1 {
                        if i == 0 && j == 0 {
                            continue;
                        }
                        let new_x = x + i;
                        let new_y = y + j;
                        if new_x < 0 || new_x >= n || new_y < 0 || new_y >= n {
                            continue;
                        }
                        if grid[new_x as usize][new_y as usize] == 1 {
                            continue;
                        }
                        if visited.contains(&(new_x, new_y)) {
                            continue;
                        }
                        queue.push_back((new_x, new_y));
                        visited.insert((new_x, new_y));
                    }
                }
            }
            step += 1;
        }
        return -1;
    }
}
