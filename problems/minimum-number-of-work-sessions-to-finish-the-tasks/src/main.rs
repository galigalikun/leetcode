fn main() {
    assert_eq!(Solution::min_sessions(vec![1, 2, 3], 3), 2);
    assert_eq!(Solution::min_sessions(vec![3, 1, 3, 1, 1], 8), 2);
    assert_eq!(Solution::min_sessions(vec![1, 2, 3, 4, 5], 15), 1);
}

struct Solution;
impl Solution {
    pub fn min_sessions(tasks: Vec<i32>, session_time: i32) -> i32 {
        let n = tasks.len();
        let mut dp = vec![i32::MAX; 1 << n];
        dp[0] = 0;
        let mut sum = vec![0; 1 << n];
        for mask in 0..(1 << n) {
            for i in 0..n {
                if (mask & (1 << i)) != 0 {
                    sum[mask] += tasks[i];
                }
            }
        }
        for mask in 0..(1 << n) {
            let mut sub = mask;
            while sub > 0 {
                if sum[sub] <= session_time {
                    dp[mask] = dp[mask].min(dp[mask ^ sub] + 1);
                }
                sub = (sub - 1) & mask;
            }
        }
        dp[(1 << n) - 1]
    }
}
