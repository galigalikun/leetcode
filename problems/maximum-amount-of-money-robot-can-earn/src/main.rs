fn main() {
    assert_eq!(
        Solution::maximum_amount(
            vec![[0, 1, -1], [1, -2, 3], [2, -3, 4]]
                .iter()
                .map(|&x| x.to_vec())
                .collect()
        ),
        8
    );
    assert_eq!(
        Solution::maximum_amount(
            vec![[10, 10, 10], [10, 10, 10]]
                .iter()
                .map(|&x| x.to_vec())
                .collect()
        ),
        40
    );
}

struct Solution;
impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![0; coins[0].len()]; coins.len()];
        for i in 0..coins.len() {
            for j in 0..coins[0].len() {
                dp[i][j] = coins[i][j]
                    + if i > 0 && j > 0 {
                        dp[i - 1][j].max(dp[i][j - 1])
                    } else if i > 0 {
                        dp[i - 1][j]
                    } else if j > 0 {
                        dp[i][j - 1]
                    } else {
                        0
                    };
            }
        }
        dp[coins.len() - 1][coins[0].len() - 1]
    }
}
