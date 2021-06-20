fn main() {
    assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    assert_eq!(Solution::coin_change(vec![2], 3), -1);
    assert_eq!(Solution::coin_change(vec![1], 0), 0);
    assert_eq!(Solution::coin_change(vec![1], 1), 1);
    assert_eq!(Solution::coin_change(vec![1], 2), 2);
    assert_eq!(Solution::coin_change(vec![2, 5, 10, 1], 27), 4);
    assert_eq!(Solution::coin_change(vec![186, 419, 83, 408], 6249), 20);
    assert_eq!(Solution::coin_change(vec![384, 324, 196, 481], 285), -1);
}

pub struct Solution {}
// https://engineer.yeele.net/algorithm/leetcode-medium-322-coin-change/
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        let mut dp: Vec<Vec<Option<i32>>> = vec![vec![Some(0); amount as usize]; coins.len()];
        for i in 0..dp.len() {
            for j in 0..dp[i].len() {
                let coin = coins[i];
                if i == 0 {
                    dp[i][j] = if (j as i32 + 1) % coin == 0 {
                        Some((j as i32 + 1) / coin)
                    } else {
                        None
                    };
                } else {
                    dp[i][j] = if j as i32 - coin >= 0 {
                        if let (Some(a), Some(b)) = (dp[i][j - coin as usize], dp[i - 1][j]) {
                            Some(std::cmp::min(a+1, b))
                        } else if let Some(a) = dp[i][j - coin as usize] {
                            Some(a+1)
                        } else if let Some(b) = dp[i - 1][j] {
                            Some(b)
                        } else {
                            None
                        }
                    } else if coin == j as i32 + 1 {
                        Some(std::cmp::min(
                            1,
                            dp[i - 1][j].unwrap_or_else(|| std::i32::MAX),
                        ))
                    } else {
                        dp[i - 1][j]
                    };
                }
            }
        }

        return dp.last().unwrap().last().unwrap().unwrap_or_else(|| -1);
    }
}
