fn main() {
    assert_eq!(
        Solution::number_of_right_triangles(vec![
            vec![1, 0, 0, 0],
            vec![0, 1, 0, 1],
            vec![1, 0, 0, 0]
        ]),
        0
    );
    assert_eq!(
        Solution::number_of_right_triangles(vec![vec![1, 0, 1], vec![1, 0, 0], vec![1, 0, 0]]),
        2
    );
}

struct Solution;
impl Solution {
    pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
        let mut row_counts = vec![0; grid.len()];
        let mut col_counts = vec![0; grid[0].len()];
        let mut total_points = 0;
        for (i, row) in grid.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                if val == 1 {
                    row_counts[i] += 1;
                    col_counts[j] += 1;
                    total_points += 1;
                }
            }
        }
        if total_points < 3 {
            // Not enough points to form a triangle
            return 0;
        }
        let mut triangle_count = 0i64;
        for (i, row) in grid.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                if val == 1 {
                    let row_points = row_counts[i] - 1; // Exclude the current point
                    let col_points = col_counts[j] - 1; // Exclude the current point
                    triangle_count += (row_points as i64) * (col_points as i64);
                }
            }
        }
        triangle_count
    }
}
