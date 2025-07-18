fn main() {
    assert_eq!(
        Solution::rotate_grid(vec![vec![40, 10], vec![30, 20],], 1),
        vec![vec![10, 20], vec![40, 30],]
    );
    assert_eq!(
        Solution::rotate_grid(
            vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 16],
            ],
            2
        ),
        vec![
            vec![3, 4, 8, 12],
            vec![2, 11, 10, 16],
            vec![1, 7, 6, 15],
            vec![5, 9, 13, 14],
        ]
    );
}

struct Solution;
impl Solution {
    pub fn rotate_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut grid = grid;
        let (m, n) = (grid.len(), grid[0].len());
        let layers = std::cmp::min(m, n) / 2;

        for layer in 0..layers {
            let mut values = vec![];
            let (row_start, row_end) = (layer, m - layer - 1);
            let (col_start, col_end) = (layer, n - layer - 1);

            // Collect the values in the current layer
            for col in col_start..=col_end {
                values.push(grid[row_start][col]);
            }
            for row in row_start + 1..=row_end {
                values.push(grid[row][col_end]);
            }
            for col in (col_start..col_end).rev() {
                values.push(grid[row_end][col]);
            }
            for row in (row_start + 1..row_end).rev() {
                values.push(grid[row][col_start]);
            }

            // Rotate the values
            let len = values.len();
            let k = k as usize % len;
            let rotated_values: Vec<i32> = values[len - k..]
                .iter()
                .cloned()
                .chain(values[..len - k].iter().cloned())
                .collect();

            // Place the rotated values back into the grid
            let mut index = 0;
            for col in col_start..=col_end {
                grid[row_start][col] = rotated_values[index];
                index += 1;
            }
            for row in row_start + 1..=row_end {
                grid[row][col_end] = rotated_values[index];
                index += 1;
            }
            for col in (col_start..col_end).rev() {
                grid[row_end][col] = rotated_values[index];
                index += 1;
            }
            for row in (row_start + 1..row_end).rev() {
                grid[row][col_start] = rotated_values[index];
                index += 1;
            }
        }

        grid
    }
}
