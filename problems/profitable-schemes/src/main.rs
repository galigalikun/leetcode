fn main() {
    assert_eq!(Solution::profitable_schemes(5, 3, vec![2,2], vec![2,3]), 2);
    assert_eq!(Solution::profitable_schemes(10, 5, vec![2,3,5], vec![6,7,8]), 7);
}

struct Solution;
impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let max_members = n as usize;
        let target_profit = min_profit as usize;
        let mut dp = vec![vec![0_i64; target_profit + 1]; max_members + 1];
        dp[0][0] = 1;

        for (&required_members, &earned_profit) in group.iter().zip(profit.iter()) {
            let required_members = required_members as usize;
            let earned_profit = earned_profit as usize;

            for members in (required_members..=max_members).rev() {
                for current_profit in (0..=target_profit).rev() {
                    let next_profit = (current_profit + earned_profit).min(target_profit);
                    dp[members][next_profit] =
                        (dp[members][next_profit] + dp[members - required_members][current_profit]) % MOD;
                }
            }
        }

        let mut schemes = 0_i64;
        for row in &dp {
            schemes = (schemes + row[target_profit]) % MOD;
        }

        schemes as i32
    }
}
