fn main() {
    assert_eq!(Solution::stone_game_ii(vec![2, 7, 9, 4, 4]), 10);
    assert_eq!(Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 100]), 104);
    assert_eq!(Solution::stone_game_ii(vec![1]), 0);
}

struct Solution;
impl Solution {
    fn dfs(dp: &mut Vec<Vec<i32>>, sum: &Vec<i32>, piles: &Vec<i32>, i: usize, m: usize) -> i32 {
        if i == piles.len() {
            return 0;
        }
        if dp[i][m] != 0 {
            return dp[i][m];
        }
        let mut res = 0;
        for x in 1..=2 * m {
            if i + x > piles.len() {
                break;
            }
            res = res.max(sum[i] - Solution::dfs(dp, sum, piles, i + x, x.max(m)));
        }
        dp[i][m] = res;
        return res;
    }
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; piles.len()]; piles.len()];
        let mut sum = vec![0; piles.len()];
        sum[piles.len() - 1] = piles[piles.len() - 1];
        for i in (0..piles.len() - 1).rev() {
            sum[i] = sum[i + 1] + piles[i];
        }
        return Solution::dfs(&mut dp, &sum, &piles, 0, 1);
    }
}
