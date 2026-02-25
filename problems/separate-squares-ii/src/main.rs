fn main() {
    assert_eq!(
        Solution::separate_squares(
            vec![[0, 0, 1], [2, 2, 1]]
                .iter()
                .map(|&x| x.to_vec())
                .collect()
        ),
        1.0
    );
    assert_eq!(
        Solution::separate_squares(
            vec![[0, 0, 2], [1, 1, 1]]
                .iter()
                .map(|&x| x.to_vec())
                .collect()
        ),
        1.0
    );
}

struct Solution;
impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let mut max_x = 0;
        let mut max_y = 0;
        for square in &squares {
            max_x = max_x.max(square[0] + square[2]);
            max_y = max_y.max(square[1] + square[2]);
        }
        let mut grid = vec![vec![0; max_y as usize]; max_x as usize];
        for square in &squares {
            for i in square[0]..square[0] + square[2] {
                for j in square[1]..square[1] + square[2] {
                    grid[i as usize][j as usize] += 1;
                }
            }
        }
        let mut count = 0;
        for i in 0..max_x {
            for j in 0..max_y {
                if grid[i as usize][j as usize] > 1 {
                    count += 1;
                }
            }
        }
        if count > 0 {
            return count as f64 / (max_x * max_y) as f64;
        }
        return 0.0;
    }
}
