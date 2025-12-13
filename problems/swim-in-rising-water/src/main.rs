fn main() {
    assert_eq!(Solution::swim_in_water(vec![vec![0, 2], vec![1, 3]]), 3);
    assert_eq!(
        Solution::swim_in_water(vec![
            vec![0, 1, 2, 3, 4],
            vec![24, 23, 22, 21, 5],
            vec![12, 13, 14, 15, 16],
            vec![11, 17, 18, 19, 20],
            vec![10, 9, 8, 7, 6]
        ]),
        16
    );
    assert_eq!(Solution::swim_in_water(vec![vec![3, 2], vec![0, 1]]), 3);
}

struct Solution {}
impl Solution {
    fn can_reach(
        grid: &Vec<Vec<i32>>,
        visited: &mut Vec<Vec<bool>>,
        x: usize,
        y: usize,
        t: i32,
    ) -> bool {
        let n = grid.len();
        let m = grid[0].len();
        if x == n - 1 && y == m - 1 {
            return true;
        }
        visited[x][y] = true;
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in directions {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            if new_x >= 0 && new_x < n as i32 && new_y >= 0 && new_y < m as i32 {
                let new_x_usize = new_x as usize;
                let new_y_usize = new_y as usize;
                if !visited[new_x_usize][new_y_usize] && grid[new_x_usize][new_y_usize] <= t {
                    if Self::can_reach(grid, visited, new_x_usize, new_y_usize, t) {
                        return true;
                    }
                }
            }
        }
        return false;
    }
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut left = 0;
        let mut right = n as i32 * m as i32 - 1;
        while left < right {
            let mid = (left + right) / 2;
            let mut visited = vec![vec![false; m]; n];
            if Self::can_reach(&grid, &mut visited, 0, 0, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        return left;
    }
}
