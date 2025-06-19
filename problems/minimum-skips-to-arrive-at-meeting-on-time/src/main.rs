fn main() {
    assert_eq!(Solution::min_skips(vec![1, 3, 2], 4, 2), 1);
    assert_eq!(Solution::min_skips(vec![7, 3, 5, 5], 2, 10), 2);
    assert_eq!(Solution::min_skips(vec![7, 3, 5, 5], 1, 10), -1);
}

struct Solution;
impl Solution {
    fn dp(dist: &Vec<i32>, speed: i32, hours_before: i32) -> i32 {
        let n = dist.len();
        let mut dp = vec![vec![i32::MAX; n + 1]; n + 1];
        dp[0][0] = 0;

        for i in 0..n {
            for j in 0..=i {
                if dp[i][j] == i32::MAX {
                    continue;
                }
                let time = (dp[i][j] + dist[i] + speed - 1) / speed; // ceil division
                if time <= hours_before {
                    dp[i + 1][j] = dp[i + 1][j].min(time);
                }
                if j < n {
                    dp[i + 1][j + 1] = dp[i + 1][j + 1].min(time);
                }
            }
        }

        for j in 0..=n {
            if dp[n][j] <= hours_before {
                return j as i32;
            }
        }
        -1
    }
    pub fn min_skips(dist: Vec<i32>, speed: i32, hours_before: i32) -> i32 {
        return Self::dp(&dist, speed, hours_before);
    }
}
