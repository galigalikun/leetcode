fn main() {
    assert_eq!(Solution::cut_off_tree(vec![vec![1,2,3],vec![0,0,4],vec![7,6,5]]), 6);
    assert_eq!(Solution::cut_off_tree(vec![vec![1,2,3],vec![0,0,0],vec![7,6,5]]), -1);
    assert_eq!(Solution::cut_off_tree(vec![vec![2,3,4],vec![0,0,5],vec![8,7,6]]), 6);
}

// https://ttzztt.gitbooks.io/lc/content/cut-off-trees-for-golf-event.html
struct Solution{}
impl Solution {
    fn shortest_path(forest: &[Vec<i32>], start_row: usize, start_col: usize, target_row: usize, target_col: usize) -> i32 {
        if start_row == target_row && start_col == target_col {
            return 0;
        }

        use std::collections::VecDeque;

        let directions = [(-1_i32, 0_i32), (1, 0), (0, -1), (0, 1)];
        let row_count = forest.len();
        let col_count = forest[0].len();

        let mut visited = vec![vec![false; col_count]; row_count];
        visited[start_row][start_col] = true;

        let mut queue = VecDeque::new();
        queue.push_back((start_row, start_col, 0_i32));

        while let Some((row, col, dist)) = queue.pop_front() {
            for (dr, dc) in directions {
                let next_row = row as i32 + dr;
                let next_col = col as i32 + dc;

                if next_row < 0
                    || next_row >= row_count as i32
                    || next_col < 0
                    || next_col >= col_count as i32
                {
                    continue;
                }

                let nr = next_row as usize;
                let nc = next_col as usize;

                if visited[nr][nc] || forest[nr][nc] == 0 {
                    continue;
                }

                if nr == target_row && nc == target_col {
                    return dist + 1;
                }

                visited[nr][nc] = true;
                queue.push_back((nr, nc, dist + 1));
            }
        }

        -1
    }

    pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        if forest.is_empty() || forest[0].is_empty() {
            return 0;
        }

        let mut trees = Vec::new();
        for row in 0..forest.len() {
            for col in 0..forest[row].len() {
                if forest[row][col] > 1 {
                    trees.push((forest[row][col], row, col));
                }
            }
        }
        trees.sort_by_key(|tree| tree.0);

        let mut current_row = 0;
        let mut current_col = 0;
        let mut total_steps = 0;

        for (_, target_row, target_col) in trees {
            let steps = Solution::shortest_path(&forest, current_row, current_col, target_row, target_col);
            if steps == -1 {
                return -1;
            }

            total_steps += steps;
            current_row = target_row;
            current_col = target_col;
        }

        total_steps
    }
}
