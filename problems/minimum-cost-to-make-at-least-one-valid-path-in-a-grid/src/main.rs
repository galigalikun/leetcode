fn main() {
    assert_eq!(
        Solution::min_cost(vec![
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2],
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2]
        ]),
        3
    );
    assert_eq!(
        Solution::min_cost(vec![vec![1, 1, 3], vec![3, 2, 2], vec![1, 1, 4]]),
        0
    );
    assert_eq!(Solution::min_cost(vec![vec![1, 2], vec![4, 3]]), 1);
}

struct Solution;
impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let grid = grid;
        let mut queue = std::collections::VecDeque::new();
        let mut cost = vec![vec![std::i32::MAX; grid[0].len()]; grid.len()];
        let dir = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        queue.push_back((0, 0, 0));
        cost[0][0] = 0;
        while !queue.is_empty() {
            let (x, y, c) = queue.pop_front().unwrap();
            for i in 0..4 {
                let (mut x, mut y) = (x as i32, y as i32);
                let mut c = c;
                while x >= 0 && y >= 0 && x < grid.len() as i32 && y < grid[0].len() as i32 {
                    if cost[x as usize][y as usize] <= c {
                        break;
                    }
                    cost[x as usize][y as usize] = c;
                    queue.push_back((x, y, c));
                    x += dir[i].0;
                    y += dir[i].1;
                    c += 1;
                }
            }
        }
        if cost[grid.len() - 1][grid[0].len() - 1] == std::i32::MAX {
            return -1;
        }
        return cost[grid.len() - 1][grid[0].len() - 1];
    }
}
