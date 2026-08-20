use std::collections::VecDeque;

fn main() {
    assert_eq!(Solution::shortest_path_all_keys(vec!["@.a..".to_string(),"###.#".to_string(),"b.A.B".to_string()]), 8);
    assert_eq!(Solution::shortest_path_all_keys(vec!["@..aA".to_string(),"..B#.".to_string(),"....b".to_string()]), 6);
    assert_eq!(Solution::shortest_path_all_keys(vec!["@Aa".to_string()]), -1);
}

struct Solution;
impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut board = Vec::with_capacity(rows);

        for row in &grid {
            board.push(row.as_bytes().to_vec());
        }

        let mut start = (0usize, 0usize);
        let mut all_keys_mask: u8 = 0;

        for (r, row) in board.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                if cell == b'@' {
                    start = (r, c);
                }
                if (b'a'..=b'f').contains(&cell) {
                    all_keys_mask |= 1 << (cell - b'a');
                }
            }
        }

        let mut visited = vec![vec![vec![false; 64]; cols]; rows];
        let mut queue = VecDeque::new();
        queue.push_back((start.0, start.1, 0u8, 0i32));
        visited[start.0][start.1][0] = true;

        let directions = [(1isize, 0isize), (-1, 0), (0, 1), (0, -1)];

        while let Some((r, c, keys_mask, steps)) = queue.pop_front() {
            if keys_mask == all_keys_mask {
                return steps;
            }

            for (dr, dc) in directions {
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                if nr < 0 || nr >= rows as isize || nc < 0 || nc >= cols as isize {
                    continue;
                }

                let nr = nr as usize;
                let nc = nc as usize;
                let cell = board[nr][nc];
                if cell == b'#' {
                    continue;
                }

                if (b'A'..=b'F').contains(&cell) {
                    let need = 1 << (cell - b'A');
                    if keys_mask & need == 0 {
                        continue;
                    }
                }

                let mut next_mask = keys_mask;
                if (b'a'..=b'f').contains(&cell) {
                    next_mask |= 1 << (cell - b'a');
                }

                if visited[nr][nc][next_mask as usize] {
                    continue;
                }
                visited[nr][nc][next_mask as usize] = true;
                queue.push_back((nr, nc, next_mask, steps + 1));
            }
        }

        -1
    }
}
