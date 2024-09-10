fn main() {
    assert_eq!(
        Solution::min_cost("abaac".to_string(), vec![1, 2, 3, 4, 5]),
        3
    );
    assert_eq!(Solution::min_cost("abc".to_string(), vec![1, 2, 3]), 0);
    assert_eq!(
        Solution::min_cost("aabaa".to_string(), vec![1, 2, 3, 4, 1]),
        2
    );
}

struct Solution;
impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; 26]; colors.len() + 1];
        let mut res = 0;
        for i in 0..colors.len() {
            let c = colors.chars().nth(i).unwrap() as usize - 'a' as usize;
            for j in 0..26 {
                dp[i + 1][j] = dp[i][j];
            }
            dp[i + 1][c] += 1;
            let mut min_cost = needed_time[i];
            for j in 0..26 {
                min_cost =
                    min_cost.min(dp[i + 1][j] + needed_time[i] * (dp[i + 1][j] / needed_time[i]));
            }
            res += min_cost;
        }
        res
    }
}
