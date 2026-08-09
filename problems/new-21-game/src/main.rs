fn main() {
    assert!((Solution::new21_game(10, 1, 10) - 1.0).abs() < 1e-9);
    assert!((Solution::new21_game(6, 1, 10) - 0.6).abs() < 1e-9);
    assert!((Solution::new21_game(21, 17, 10) - 0.73278).abs() < 1e-5);
}

struct Solution {}
impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        if k == 0 || n >= k - 1 + max_pts {
            return 1.0;
        }

        let n = n as usize;
        let k = k as usize;
        let max_pts = max_pts as usize;

        let mut dp = vec![0.0; n + 1];
        dp[0] = 1.0;

        let mut window_sum = 1.0;
        let mut result = 0.0;

        for i in 1..=n {
            dp[i] = window_sum / max_pts as f64;

            if i < k {
                window_sum += dp[i];
            } else {
                result += dp[i];
            }

            if i >= max_pts && i - max_pts < k {
                window_sum -= dp[i - max_pts];
            }
        }

        result
    }
}
