fn main() {
    assert_eq!(
        Solution::num_submat(vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]]),
        13
    );
    assert_eq!(
        Solution::num_submat(vec![vec![0, 1, 1, 0], vec![0, 1, 1, 1], vec![1, 1, 1, 0]]),
        24
    );
}

struct Solution;
impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let n = mat.len();
        let m = mat[0].len();
        let mut dp = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                if mat[i][j] == 1 {
                    dp[i][j] = if j == 0 { 1 } else { dp[i][j - 1] + 1 };
                    let mut min = dp[i][j];
                    for k in (0..=i).rev() {
                        min = min.min(dp[k][j]);
                        res += min;
                    }
                }
            }
        }
        return res;
    }
}
