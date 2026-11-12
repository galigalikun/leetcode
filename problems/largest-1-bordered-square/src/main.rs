fn main() {
    assert_eq!(
        Solution::largest1_bordered_square(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
        9
    );
    assert_eq!(
        Solution::largest1_bordered_square(vec![vec![1, 1, 0, 0]]),
        1
    );
}

struct Solution;
impl Solution {
    pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        if m == 0 || n == 0 {
            return 0;
        }
        
        let mut horizontal = vec![vec![0; n]; m];
        let mut vertical = vec![vec![0; n]; m];
        let mut max_size = 0;
        
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    horizontal[i][j] = if j > 0 { horizontal[i][j - 1] + 1 } else { 1 };
                    vertical[i][j] = if i > 0 { vertical[i - 1][j] + 1 } else { 1 };
                } else {
                    horizontal[i][j] = 0;
                    vertical[i][j] = 0;
                }
            }
        }
        
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    let mut side_len = (horizontal[i][j].min(vertical[i][j])) as i32;
                    while side_len > max_size {
                        let top_row = (i as i32 - side_len + 1) as usize;
                        let left_col = (j as i32 - side_len + 1) as usize;
                        
                        if (i as i32 - side_len + 1) >= 0 && (j as i32 - side_len + 1) >= 0 {
                            if horizontal[top_row][j] >= side_len as usize && vertical[i][left_col] >= side_len as usize {
                                max_size = side_len;
                                break;
                            }
                        }
                        side_len -= 1;
                    }
                }
            }
        }
        
        (max_size * max_size) as i32
    }
}
