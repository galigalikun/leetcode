fn main() {
    assert_eq!(Solution::num_squares(12), 3);
    assert_eq!(Solution::num_squares(13), 2);
}

struct Solution {}
// https://qiita.com/KueharX/items/72aa31599033ff4aca70
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut dp = Vec::new();
        for i in 0..n + 1 {
            dp.push(i);
        }

        for i in 2..(n + 1) as usize {
            for j in 1..((i as f64).sqrt() + 1.0) as usize {
                dp[i] = std::cmp::min(dp[i], dp[i - j * j] + 1);
            }
        }
        return dp[n as usize];
    }
}
