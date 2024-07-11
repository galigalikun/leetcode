fn main() {
    assert_eq!(Solution::winner_square_game(1), true);
    assert_eq!(Solution::winner_square_game(2), false);
    assert_eq!(Solution::winner_square_game(4), true);
}

struct Solution;
impl Solution {
    fn dfs(n: i32, dp: &mut Vec<i32>) -> bool {
        if dp[n as usize] != 0 {
            return dp[n as usize] == 1;
        }
        for i in 1..=n {
            let square = i * i;
            if square > n {
                break;
            }
            if !Self::dfs(n - square, dp) {
                dp[n as usize] = 1;
                return true;
            }
        }
        dp[n as usize] = -1;
        return false;
    }
    pub fn winner_square_game(n: i32) -> bool {
        return Self::dfs(n, &mut vec![0; n as usize + 1]);
    }
}
