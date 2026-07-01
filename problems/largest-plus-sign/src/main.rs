fn main() {
    assert_eq!(Solution::order_of_largest_plus_sign(5, vec![vec![4,2]]), 2);
    assert_eq!(Solution::order_of_largest_plus_sign(0, vec![vec![0,0]]), 0);
}

struct Solution{}
impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        if n <= 0 {
            return 0;
        }

        let n = n as usize;
        let mut dp = vec![vec![n as i32; n]; n];

        for mine in mines {
            let r = mine[0] as usize;
            let c = mine[1] as usize;
            dp[r][c] = 0;
        }

        for r in 0..n {
            let mut run = 0;
            for c in 0..n {
                if dp[r][c] == 0 {
                    run = 0;
                } else {
                    run += 1;
                }
                dp[r][c] = dp[r][c].min(run);
            }

            run = 0;
            for c in (0..n).rev() {
                if dp[r][c] == 0 {
                    run = 0;
                } else {
                    run += 1;
                }
                dp[r][c] = dp[r][c].min(run);
            }
        }

        for c in 0..n {
            let mut run = 0;
            for r in 0..n {
                if dp[r][c] == 0 {
                    run = 0;
                } else {
                    run += 1;
                }
                dp[r][c] = dp[r][c].min(run);
            }

            run = 0;
            for r in (0..n).rev() {
                if dp[r][c] == 0 {
                    run = 0;
                } else {
                    run += 1;
                }
                dp[r][c] = dp[r][c].min(run);
            }
        }

        let mut best = 0;
        for r in 0..n {
            for c in 0..n {
                best = best.max(dp[r][c]);
            }
        }

        best
    }
}
