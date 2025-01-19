fn main() {
    assert_eq!(Solution::check_ways(vec![vec![1, 2], vec![2, 3]]), 1);
    assert_eq!(
        Solution::check_ways(vec![vec![1, 2], vec![2, 3], vec![1, 3]]),
        2
    );
    assert_eq!(
        Solution::check_ways(vec![vec![1, 2], vec![2, 3], vec![2, 4], vec![1, 5]]),
        0
    );
}

struct Solution;
impl Solution {
    pub fn check_ways(pairs: Vec<Vec<i32>>) -> i32 {
        let n = pairs.iter().map(|x| x.iter().max().unwrap()).max().unwrap() + 1;
        let mut g = vec![vec![]; n as usize];
        for p in pairs {
            g[p[0] as usize].push(p[1] as usize);
            g[p[1] as usize].push(p[0] as usize);
        }
        let mut dp = vec![vec![0; 3]; n as usize];
        let mut vis = vec![0; n as usize];
        let mut ans = 0;
        for i in 0..n as usize {
            if vis[i] == 0 {
                let mut cnt = 0;
                let mut a = 1;
                let mut b = 1;
                vis[i] = 1;
                for &j in &g[i] {
                    if vis[j] == 0 {
                        let mut c = 0;
                        let mut d = 0;
                        vis[j] = 1;
                        for &k in &g[j] {
                            if vis[k] == 0 {
                                vis[k] = 1;
                                c += dp[k][0];
                                d += dp[k][1];
                            }
                        }
                        if c == 0 || d == 0 {
                            return 0;
                        }
                        a = a * c % 1_000_000_007;
                        b = b * d % 1_000_000_007;
                        cnt += 1;
                    }
                }
                if cnt == 0 {
                    dp[i][0] = 1;
                    dp[i][1] = 1;
                    dp[i][2] = 1;
                } else {
                    dp[i][0] = a;
                    dp[i][1] = b;
                    dp[i][2] = (a + b) % 1_000_000_007;
                }
                ans = (ans + dp[i][2]) % 1_000_000_007;
            }
        }
        if ans == 0 {
            return 2;
        }
        ans
    }
}
