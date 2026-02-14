fn main() {
    assert_eq!(Solution::champagne_tower(1, 1, 1), 0.0);
    assert_eq!(Solution::champagne_tower(2, 1, 1), 0.5);
    assert_eq!(Solution::champagne_tower(100000009, 33, 17), 1.0);
}

struct Solution {}
impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut dp = vec![vec![0.0; 101]; 101];
        dp[0][0] = poured as f64;
        for i in 0..=query_row as usize {
            for j in 0..=i {
                let overflow = (dp[i][j] - 1.0).max(0.0) / 2.0;
                if i + 1 <= query_row as usize {
                    dp[i + 1][j] += overflow;
                    dp[i + 1][j + 1] += overflow;
                }
            }
        }
        dp[query_row as usize][query_glass as usize].min(1.0)
    }
}
