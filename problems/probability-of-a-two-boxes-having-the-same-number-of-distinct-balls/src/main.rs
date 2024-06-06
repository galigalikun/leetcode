fn main() {
    assert_eq!(Solution::get_probability(vec![1, 1]), 1.0);
    assert_eq!(Solution::get_probability(vec![2, 1, 1]), 0.66667);
    assert_eq!(Solution::get_probability(vec![1, 2, 1, 2]), 0.6);
}

struct Solution;
impl Solution {
    pub fn get_probability(balls: Vec<i32>) -> f64 {
        let mut total = 0;
        for &b in &balls {
            total += b;
        }
        let mut dp = vec![vec![0.0; total as usize + 1]; balls.len() + 1];
        dp[0][0] = 1.0;
        for i in 1..=balls.len() {
            for j in 0..=total as usize {
                if j < balls[i - 1] as usize {
                    dp[i][j] = dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i - 1][j] + dp[i - 1][j - balls[i - 1] as usize];
                }
            }
        }
        let mut ans = 0.0;
        for i in 0..=total {
            let mut red = 0;
            let mut blue = 0;
            for j in 0..balls.len() {
                if j < balls.len() - 1 {
                    red += balls[j];
                } else {
                    blue += balls[j];
                }
            }
            if i < red || i > total - blue {
                continue;
            }
            let red_prob = dp[balls.len()][i as usize] / dp[balls.len()][total as usize];
            let blue_prob = dp[balls.len()][(total - i) as usize] / dp[balls.len()][total as usize];
            ans += red_prob * blue_prob;
        }
        return ans;
    }
}
