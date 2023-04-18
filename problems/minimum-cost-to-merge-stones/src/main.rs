fn main() {
    assert_eq!(Solution::merge_stones(vec![3,2,4,1], 2), 20);
    assert_eq!(Solution::merge_stones(vec![3,2,4,1], 3), -1);
    assert_eq!(Solution::merge_stones(vec![3,5,1,2,6], 3), 25);
}

struct Solution;
impl Solution {
    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        let n = stones.len();
        if (n as i32 - 1) % (k - 1) != 0 {
            return -1;
        }
        let mut dp = vec![vec![0; n]; n];
        let mut sum = vec![0; n + 1];
        for i in 0..n {
            sum[i + 1] = sum[i] + stones[i];
        }
        for m in k as usize..=n {
            for i in 0..=n - m {
                let j = i + m - 1;
                dp[i][j] = std::i32::MAX;
                for mid in i..j {
                    dp[i][j] = dp[i][j].min(dp[i][mid] + dp[mid + 1][j]);
                }
                if (j - i) % (k as usize - 1) == 0 {
                    dp[i][j] += sum[j + 1] - sum[i];
                }
            }
        }
        dp[0][n - 1]
    }
}
