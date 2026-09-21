use std::collections::VecDeque;

fn main() {
    assert_eq!(Solution::shortest_bridge(vec![vec![0, 1], vec![1, 0]]), 1);
    assert_eq!(
        Solution::shortest_bridge(vec![vec![0, 1, 0], vec![0, 0, 0], vec![0, 0, 1]]),
        2
    );
    assert_eq!(
        Solution::shortest_bridge(vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1]
        ]),
        1
    );
}

struct Solution;
impl Solution {
    pub fn shortest_bridge(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut found = false;

        for row in 0..n {
            if found {
                break;
            }

            for col in 0..n {
                if grid[row][col] == 1 {
                    Self::mark_island(row, col, &mut grid, &mut queue);
                    found = true;
                    break;
                }
            }
        }

        let directions: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut steps = 0;

        while !queue.is_empty() {
            let level_size = queue.len();

            for _ in 0..level_size {
                let (row, col) = queue
                    .pop_front()
                    .expect("queue contains level_size elements");

                for &(dr, dc) in &directions {
                    let next_row = row as isize + dr;
                    let next_col = col as isize + dc;

                    if next_row < 0
                        || next_col < 0
                        || next_row >= n as isize
                        || next_col >= n as isize
                    {
                        continue;
                    }

                    let nr = next_row as usize;
                    let nc = next_col as usize;

                    if grid[nr][nc] == 1 {
                        return steps;
                    }

                    if grid[nr][nc] == 0 {
                        grid[nr][nc] = 2;
                        queue.push_back((nr, nc));
                    }
                }
            }

            steps += 1;
        }

        -1
    }

    fn mark_island(
        row: usize,
        col: usize,
        grid: &mut [Vec<i32>],
        queue: &mut VecDeque<(usize, usize)>,
    ) {
        let n = grid.len() as isize;
        let mut stack = vec![(row, col)];
        let directions: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        while let Some((r, c)) = stack.pop() {
            if grid[r][c] != 1 {
                continue;
            }

            grid[r][c] = 2;
            queue.push_back((r, c));

            for &(dr, dc) in &directions {
                let next_row = r as isize + dr;
                let next_col = c as isize + dc;

                if next_row < 0 || next_col < 0 || next_row >= n || next_col >= n {
                    continue;
                }

                let nr = next_row as usize;
                let nc = next_col as usize;

                if grid[nr][nc] == 1 {
                    stack.push((nr, nc));
                }
            }
        }
    }
}
