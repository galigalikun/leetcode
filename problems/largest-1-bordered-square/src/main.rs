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
        let mut max = 0;
        let mut dp = vec![vec![vec![0; 2]; grid[0].len()]; grid.len()];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    dp[i][j][0] = if i > 0 { dp[i - 1][j][0] + 1 } else { 1 };
                    dp[i][j][1] = if j > 0 { dp[i][j - 1][1] + 1 } else { 1 };
                    let mut min = dp[i][j][0].min(dp[i][j][1]);
                    while min > max {
                        if dp[i][j - min + 1][1] >= min && dp[i - min + 1][j][0] >= min {
                            max = min;
                        }
                        min -= 1;
                    }
                }
            }
        }
        return (max * max) as i32;
    }
}
