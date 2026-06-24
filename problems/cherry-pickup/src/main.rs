fn main() {
    assert_eq!(Solution::cherry_pickup(vec![vec![0, 1, -1], vec![1, 0, -1], vec![1, 1, 1]]), 5);
    assert_eq!(Solution::cherry_pickup(vec![vec![1,1,-1],vec![1,-1,1],vec![-1,1,1]]), 0);
}

struct Solution;
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        // Model the round trip as two people walking simultaneously from (0,0)
        // to (n-1,n-1), both moving only right or down. After the same number
        // of steps k, person 1 is at (r1, c1) and person 2 at (r2, c2) with
        // r1 + c1 == r2 + c2 == k, so r1 = k - c1 and r2 = k - c2. The state
        // reduces to (k, c1, c2); we maximize the cherries the two pick up,
        // counting a shared cell only once.
        let n = grid.len();

        // dp[c1][c2] for the current step k. NEG marks an unreachable state.
        const NEG: i32 = i32::MIN;
        let mut dp = vec![vec![NEG; n]; n];
        dp[0][0] = grid[0][0];

        for k in 1..=(2 * (n - 1)) {
            let mut next = vec![vec![NEG; n]; n];
            let c1_lo = k.saturating_sub(n - 1);
            for c1 in c1_lo..=k.min(n - 1) {
                let r1 = k - c1;
                if grid[r1][c1] == -1 {
                    continue;
                }
                for c2 in c1..=k.min(n - 1) {
                    let r2 = k - c2;
                    if grid[r2][c2] == -1 {
                        continue;
                    }
                    // Best of the four move combinations from step k-1.
                    let mut best = NEG;
                    for &pc1 in &[c1, c1.wrapping_sub(1)] {
                        for &pc2 in &[c2, c2.wrapping_sub(1)] {
                            if pc1 < n && pc2 < n && dp[pc1][pc2] != NEG {
                                best = best.max(dp[pc1][pc2]);
                            }
                        }
                    }
                    if best == NEG {
                        continue;
                    }
                    let mut gained = grid[r1][c1];
                    if c1 != c2 {
                        gained += grid[r2][c2];
                    }
                    next[c1][c2] = best + gained;
                }
            }
            dp = next;
        }

        dp[n - 1][n - 1].max(0)
    }
}
