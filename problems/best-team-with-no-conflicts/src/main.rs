fn main() {
    assert_eq!(
        Solution::best_team_score(vec![1, 3, 5, 10, 15], vec![1, 2, 3, 4, 5]),
        34
    );
    assert_eq!(
        Solution::best_team_score(vec![4, 5, 6, 5], vec![2, 1, 2, 1]),
        16
    );
    assert_eq!(
        Solution::best_team_score(vec![1, 2, 3, 5], vec![8, 9, 10, 1]),
        6
    );
}

struct Solution;
impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let mut players = scores.iter().zip(ages.iter()).collect::<Vec<_>>();
        players.sort_by(|a, b| a.1.cmp(b.1).then(a.0.cmp(b.0)));
        let n = players.len();
        let mut dp = vec![0; n];
        for i in 0..n {
            dp[i] = *players[i].0;
            for j in 0..i {
                if players[j].0 <= players[i].0 {
                    dp[i] = dp[i].max(dp[j] + players[i].0);
                }
            }
        }
        *dp.iter().max().unwrap()
    }
}
