fn main() {
    assert_eq!(
        Solution::calculate_minimum_hp(vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]]),
        7
    );

    assert_eq!(Solution::calculate_minimum_hp(vec![vec![0]]), 1);
}

pub struct Solution {}
// https://gist.github.com/JyotinderSingh/c38b0b0d48dd56b75ea0d6204db988d4
impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let rows = dungeon.len();
        let cols = dungeon[0].len();
        let mut dp = vec![vec![0; cols]; rows];

        dp[rows - 1][cols - 1] = 1 - dungeon[rows - 1][cols - 1];
        dp[rows - 1][cols - 1] = if dp[rows - 1][cols - 1] <= 0 {
            1
        } else {
            dp[rows - 1][cols - 1]
        };

        for row in (0..rows).rev() {
            for col in (0..cols).rev() {
                if row == rows - 1 && col == cols - 1 {
                    continue;
                }

                let hp_down = if row == rows - 1 {
                    std::i32::MAX
                } else {
                    dp[row + 1][col] - dungeon[row][col]
                };

                let hp_right = if col == cols - 1 {
                    std::i32::MAX
                } else {
                    dp[row][col + 1] - dungeon[row][col]
                };

                let hp = std::cmp::min(hp_right, hp_down);
                dp[row][col] = if hp <= 0 { 1 } else { hp };
            }
        }

        return dp[0][0];
    }
}
