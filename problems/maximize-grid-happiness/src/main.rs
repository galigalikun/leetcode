fn main() {
    assert_eq!(Solution::get_max_grid_happiness(2, 3, 1, 2), 240);
    assert_eq!(Solution::get_max_grid_happiness(3, 1, 2, 1), 260);
    assert_eq!(Solution::get_max_grid_happiness(2, 2, 4, 0), 240);
}

struct Solution;
impl Solution {
    pub fn get_max_grid_happiness(
        m: i32,
        n: i32,
        introverts_count: i32,
        extroverts_count: i32,
    ) -> i32 {
        let mut dp = vec![vec![vec![vec![-1; 6]; 6]; 6]; 6];
        dp[0][0][0][0] = 0;
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                let mut next = vec![vec![vec![vec![-1; 6]; 6]; 6]; 6];
                for introverts in 0..=introverts_count as usize {
                    for extroverts in 0..=extroverts_count as usize {
                        for prev_in in 0..6 {
                            for prev_ex in 0..6 {
                                for k in 0..6 {
                                    if dp[introverts][extroverts][prev_in][prev_ex] == -1 {
                                        continue;
                                    }
                                    // place introvert
                                    if introverts < introverts_count as usize {
                                        let mut score =
                                            dp[introverts][extroverts][prev_in][prev_ex];
                                        if prev_in > 0 {
                                            score += 120;
                                        } else {
                                            score += 40;
                                        }
                                        if j > 0 {
                                            score += 40;
                                        }
                                        next[introverts + 1][extroverts][prev_ex][k] =
                                            next[introverts + 1][extroverts][prev_ex][k].max(score);
                                    }
                                    // place extrovert
                                    if extroverts < extroverts_count as usize {
                                        let mut score =
                                            dp[introverts][extroverts][prev_in][prev_ex];
                                        if prev_ex > 0 {
                                            score += 40;
                                        } else {
                                            score += 20;
                                        }
                                        if j > 0 {
                                            score += 40;
                                        }
                                        next[introverts][extroverts + 1][prev_in][k] =
                                            next[introverts][extroverts + 1][prev_in][k].max(score);
                                    }
                                    // place empty
                                    let mut score = dp[introverts][extroverts][prev_in][prev_ex];
                                    if j > 0 {
                                        score += 40;
                                    }
                                    next[introverts][extroverts][prev_ex][k] =
                                        next[introverts][extroverts][prev_ex][k].max(score);
                                }
                            }
                        }
                    }
                }
                dp = next;
            }
        }
        for introverts in 0..=introverts_count as usize {
            for extroverts in 0..=extroverts_count as usize {
                for prev_in in 0..6 {
                    for prev_ex in 0..6 {
                        ans = ans.max(dp[introverts][extroverts][prev_in][prev_ex]);
                    }
                }
            }
        }
        ans
    }
}
