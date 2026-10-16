fn main() {
    assert_eq!(Solution::merge_stones(vec![3, 2, 4, 1], 2), 20);
    assert_eq!(Solution::merge_stones(vec![3, 2, 4, 1], 3), -1);
    assert_eq!(Solution::merge_stones(vec![3, 5, 1, 2, 6], 3), 25);
    assert_eq!(Solution::merge_stones(vec![6, 4, 4, 6], 2), 40);
}

struct Solution;
impl Solution {
    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        let n = stones.len();
        if n <= 1 {
            return 0;
        }
        if k <= 1 {
            return -1;
        }
        if (n as i32 - 1) % (k - 1) != 0 {
            return -1;
        }

        let k = k as usize;
        let mut dp = vec![vec![0; n]; n];
        let mut sum = vec![0; n + 1];
        for i in 0..n {
            sum[i + 1] = sum[i] + stones[i];
        }

        for len in 2..=n {
            for i in 0..=n - len {
                let j = i + len - 1;
                dp[i][j] = i32::MAX;
                for mid in (i..j).step_by(k - 1) {
                    dp[i][j] = dp[i][j].min(dp[i][mid] + dp[mid + 1][j]);
                }
                if (j - i) % (k - 1) == 0 {
                    dp[i][j] += sum[j + 1] - sum[i];
                }
            }
        }
        dp[0][n - 1]
    }
}
