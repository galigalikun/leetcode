fn main() {
    assert_eq!(
        Solution::latest_day_to_cross(2, 2, vec![vec![1, 1], vec![2, 1], vec![1, 2], vec![2, 2]]),
        2
    );
    assert_eq!(
        Solution::latest_day_to_cross(2, 2, vec![vec![1, 1], vec![1, 2], vec![2, 1], vec![2, 2]]),
        1
    );
    assert_eq!(
        Solution::latest_day_to_cross(
            3,
            3,
            vec![
                vec![1, 2],
                vec![2, 1],
                vec![3, 3],
                vec![2, 2],
                vec![1, 1],
                vec![1, 3],
                vec![2, 3],
                vec![3, 2],
                vec![3, 1]
            ]
        ),
        3
    );
}

struct Solution;
impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let (mut left, mut right) = (0, cells.len() - 1);
        while left <= right {
            let mid = left + (right - left) / 2;
            let mut grid = vec![vec![0; col as usize]; row as usize];
            for i in 0..=mid {
                grid[(cells[i][0] - 1) as usize][(cells[i][1] - 1) as usize] = 1;
            }
            let mut visited = vec![vec![false; col as usize]; row as usize];
            let mut stack = vec![];
            for c in 0..col as usize {
                if grid[0][c] == 0 {
                    stack.push((0, c));
                    visited[0][c] = true;
                }
            }
            let mut can_cross = false;
            while let Some((r, c)) = stack.pop() {
                if r == (row - 1) as usize {
                    can_cross = true;
                    break;
                }
                for (dr, dc) in &directions {
                    let (nr, nc) = (r as i32 + dr, c as i32 + dc);
                    if nr >= 0
                        && nr < row
                        && nc >= 0
                        && nc < col
                        && grid[nr as usize][nc as usize] == 0
                        && !visited[nr as usize][nc as usize]
                    {
                        visited[nr as usize][nc as usize] = true;
                        stack.push((nr as usize, nc as usize));
                    }
                }
            }
            if can_cross {
                left = mid + 1;
            } else {
                if mid == 0 {
                    break;
                }
                right = mid - 1;
            }
        }
        return right as i32 + 1;
    }
}
