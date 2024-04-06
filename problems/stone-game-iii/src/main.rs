use std::vec;

fn main() {
    assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, 7]), "Bob");
    assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, -9]), "Alice");
    assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, 6]), "Tie");
}

struct Solution;
impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let n = stone_value.len();
        let mut dp = vec![0; n + 1];
        for i in (0..n).rev() {
            dp[i] = std::i32::MIN;
            let mut sum = 0;
            for k in 0..3 {
                if i + k < n {
                    sum += stone_value[i + k];
                    dp[i] = dp[i].max(sum - dp[i + k + 1]);
                }
            }
        }
        return if dp[0] > 0 {
            "Alice".to_string()
        } else if dp[0] < 0 {
            "Bob".to_string()
        } else {
            "Tie".to_string()
        };
    }
}
