fn main() {
    assert_eq!(
        Solution::color_border(vec![vec![1, 1], vec![1, 2]], 0, 0, 3),
        vec![vec![3, 3], vec![3, 2]]
    );
    assert_eq!(
        Solution::color_border(vec![vec![1, 2, 2], vec![2, 3, 2]], 0, 1, 3),
        vec![vec![1, 3, 3], vec![2, 3, 3]]
    );
    assert_eq!(
        Solution::color_border(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]], 1, 1, 2),
        vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]]
    );
}

struct Solution;
impl Solution {
    pub fn color_border(grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
        let mut grid = grid;
        let rows = grid.len();
        let cols = grid[0].len();
        let start_row = row as usize;
        let start_col = col as usize;
        let original_color = grid[start_row][start_col];

        let mut visited = vec![vec![false; cols]; rows];
        let mut stack = vec![(start_row, start_col)];
        let mut border = vec![];

        while let Some((r, c)) = stack.pop() {
            if visited[r][c] {
                continue;
            }
            visited[r][c] = true;

            let mut is_border = r == 0 || r == rows - 1 || c == 0 || c == cols - 1;
            let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

            for &(dr, dc) in &directions {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;

                if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                    is_border = true;
                    continue;
                }

                let nr = nr as usize;
                let nc = nc as usize;
                if grid[nr][nc] != original_color {
                    is_border = true;
                } else if !visited[nr][nc] {
                    stack.push((nr, nc));
                }
            }

            if is_border {
                border.push((r, c));
            }
        }

        for (r, c) in border {
            grid[r][c] = color;
        }

        grid
    }
}
