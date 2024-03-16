fn main() {
    assert_eq!(Solution::max_size_slices(vec![1, 2, 3, 4, 5, 6]), 10);
    assert_eq!(Solution::max_size_slices(vec![8, 9, 8, 6, 1, 1]), 16);
    assert_eq!(
        Solution::max_size_slices(vec![9, 5, 1, 7, 8, 4, 4, 5, 5, 8, 7, 7]),
        30
    );
}

struct Solution;
impl Solution {
    pub fn max_size_slices(slices: Vec<i32>) -> i32 {
        let n = slices.len();
        let m = n / 3;
        let mut dp = vec![vec![0; n]; m];
        for i in 0..n {
            for j in 0..m {
                if i == 0 {
                    dp[j][i] = slices[i];
                } else if i == 1 {
                    dp[j][i] = std::cmp::max(slices[i], slices[i - 1]);
                } else if j == 0 {
                    dp[j][i] = std::cmp::max(dp[j][i - 1], slices[i]);
                } else {
                    dp[j][i] = std::cmp::max(dp[j][i - 1], dp[j - 1][i - 2] + slices[i]);
                }
            }
        }
        return *dp[m - 1].iter().max().unwrap();
    }
}
