fn main() {
    assert_eq!(
        Solution::largest_magic_square(vec![
            vec![7, 1, 4, 2],
            vec![2, 3, 6, 5],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4]
        ]),
        3
    );
    assert_eq!(
        Solution::largest_magic_square(vec![vec![5, 1, 3, 1], vec![9, 3, 3, 1], vec![1, 3, 3, 8]]),
        2
    );
}

struct Solution;
impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        let mut ans = 0;
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if i + dp[i + 1][j + 1] < m && j + dp[i + 1][j + 1] < n {
                    let size = dp[i + 1][j + 1] + 1;
                    let mut valid = true;
                    let mut sum = 0;
                    for k in 0..size {
                        sum += grid[i + k][j + k];
                    }
                    for k in 0..size {
                        let mut row_sum = 0;
                        let mut col_sum = 0;
                        for l in 0..size {
                            row_sum += grid[i + k][j + l];
                            col_sum += grid[i + l][j + k];
                        }
                        if row_sum != sum || col_sum != sum {
                            valid = false;
                            break;
                        }
                    }
                    if valid {
                        for _k in 0..size {
                            let mut diag1_sum = 0;
                            let mut diag2_sum = 0;
                            for l in 0..size {
                                diag1_sum += grid[i + l][j + l];
                                diag2_sum += grid[i + l][j + size - 1 - l];
                            }
                            if diag1_sum != sum || diag2_sum != sum {
                                valid = false;
                                break;
                            }
                        }
                    }
                    if valid {
                        dp[i][j] = size;
                        ans = ans.max(size);
                    }
                }
            }
        }
        ans as i32
    }
}
