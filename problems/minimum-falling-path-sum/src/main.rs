fn main() {
    assert_eq!(Solution::min_falling_path_sum(vec![vec![2,1,3],vec![6,5,4],vec![7,8,9]]), 13);
    assert_eq!(Solution::min_falling_path_sum(vec![vec![-19,57],vec![-40,-5]]), -59);
}

struct Solution;
impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let mut dp = matrix[0].clone();

        for row in matrix.iter().skip(1) {
            let next: Vec<i32> = (0..n)
                .map(|col| {
                    let left = if col > 0 { dp[col - 1] } else { i32::MAX };
                    let up = dp[col];
                    let right = if col + 1 < n { dp[col + 1] } else { i32::MAX };

                    row[col] + left.min(up).min(right)
                })
                .collect();

            dp = next;
        }

        dp.into_iter().min().unwrap_or(0)
    }
}
