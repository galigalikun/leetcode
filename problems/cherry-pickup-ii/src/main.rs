use std::vec;

fn main() {
    assert_eq!(Solution::cherry_pickup(vec![vec![3,1,1],vec![2,5,1],vec![1,5,5],vec![2,1,1]]), 24);
    assert_eq!(Solution::cherry_pickup(vec![vec![1,0,0,0,0,0,1],vec![2,0,0,0,0,3,0],vec![2,0,9,0,0,0,0],vec![0,3,0,5,4,0,0],vec![1,0,2,3,0,0,6]]), 28);
}

struct Solution;
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut dp = vec![vec![vec![0; n]; n]; n];
        dp[0][0][0] = grid[0][0];
        for t in 1..2*n-1 {
            for i in (0..n).rev() {
                for p in (0..n).rev() {
                    for q in (0..n).rev() {
                        if t < i || t < p || t < q || t-i >= n || t-p >= n || t-q >= n {
                            continue;
                        }
                        let j = t-i;
                        if j >= n || j < p || j > q {
                            continue;
                        }
                        let mut val = grid[i][j];
                        if i != p {
                            val += grid[p][q];
                        }
                        dp[t][i][p] = val + dp[t-1][i][p].max(dp[t-1][i+1][p]).max(dp[t-1][i][p+1]).max(dp[t-1][i+1][p+1]);
                    }
                }
            }
        }
        return dp[2*n-2][n-1][n-1].max(0);
    }
}
