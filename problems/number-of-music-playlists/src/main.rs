fn main() {
    assert_eq!(Solution::num_music_playlists(3, 3, 1), 6);
    assert_eq!(Solution::num_music_playlists(2, 3, 0), 6);
    assert_eq!(Solution::num_music_playlists(2, 3, 1), 2);
}

struct Solution;
impl Solution {
    pub fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let n = n as usize;
        let goal = goal as usize;
        let k = k as usize;

        let mut dp = vec![vec![0_i64; n + 1]; goal + 1];
        dp[0][0] = 1;

        for i in 1..=goal {
            for j in 1..=n.min(i) {
                let add_new = dp[i - 1][j - 1] * (n - (j - 1)) as i64;
                let replay_old = if j > k {
                    dp[i - 1][j] * (j - k) as i64
                } else {
                    0
                };
                dp[i][j] = (add_new + replay_old) % MOD;
            }
        }

        dp[goal][n] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn sample_case_1() {
        assert_eq!(Solution::num_music_playlists(3, 3, 1), 6);
    }

    #[test]
    fn sample_case_2() {
        assert_eq!(Solution::num_music_playlists(2, 3, 0), 6);
    }

    #[test]
    fn sample_case_3() {
        assert_eq!(Solution::num_music_playlists(2, 3, 1), 2);
    }

    #[test]
    fn impossible_when_goal_less_than_n() {
        assert_eq!(Solution::num_music_playlists(3, 2, 1), 0);
    }
}
