fn main() {
    assert_eq!(
        Solution::max_points(vec![vec![1, 2, 3], vec![1, 5, 1], vec![3, 1, 1]]),
        9
    );
    assert_eq!(
        Solution::max_points(vec![vec![1, 5], vec![2, 3], vec![4, 2]]),
        11
    );
}

struct Solution;
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let n = points.len();
        let m = points[0].len();
        let mut dp = vec![vec![0; m]; n];
        for j in 0..m {
            dp[0][j] = points[0][j] as i64;
        }
        for i in 1..n {
            let mut left = vec![0; m];
            let mut right = vec![0; m];
            left[0] = dp[i - 1][0];
            for j in 1..m {
                left[j] = left[j - 1].max(dp[i - 1][j] - j as i64);
            }
            right[m - 1] = dp[i - 1][m - 1];
            for j in (0..m - 1).rev() {
                right[j] = right[j + 1].max(dp[i - 1][j] + j as i64);
            }
            for j in 0..m {
                dp[i][j] = points[i][j] as i64 + left[j].max(right[j]);
            }
        }
        let mut ans = 0;
        for j in 0..m {
            ans = ans.max(dp[n - 1][j]);
        }
        ans
    }
}
