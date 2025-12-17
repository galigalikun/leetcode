fn main() {
    assert_eq!(Solution::min_time(vec![1, 5, 2, 4], vec![5, 1, 4, 2]), 110);
    assert_eq!(Solution::min_time(vec![1, 1, 1], vec![1, 1, 1]), 5);
    assert_eq!(Solution::min_time(vec![1, 2, 3, 4], vec![1, 2]), 21);
}

struct Solution;
impl Solution {
    pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
        let n = skill.len();
        let mut dp = vec![vec![i64::MAX; 101]; n + 1];
        dp[0][0] = 0;

        for i in 1..=n {
            let s = skill[i - 1] as usize;
            let m = mana[i - 1] as i64;
            for j in 0..=100 {
                dp[i][j] = dp[i - 1][j].min(dp[i][j]);
                let nj = (j + s).min(100);
                dp[i][nj] = dp[i - 1][j].saturating_add(m).min(dp[i][nj]);
            }
        }
        dp[n][100]
    }
}
