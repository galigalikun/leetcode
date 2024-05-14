fn main() {
    assert_eq!(
        Solution::ways(
            vec!["A..".to_string(), "AAA".to_string(), "...".to_string()],
            3
        ),
        3
    );
    assert_eq!(
        Solution::ways(
            vec!["A..".to_string(), "AA.".to_string(), "...".to_string()],
            3
        ),
        1
    );
    assert_eq!(
        Solution::ways(
            vec!["A..".to_string(), "A..".to_string(), "...".to_string()],
            1
        ),
        1
    );
}

struct Solution;
impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        let mut dp = vec![vec![vec![-1; 11]; 11]; 51];
        let mut pre = vec![vec![0; 11]; 11];
        let mut sum = vec![vec![0; 11]; 11];
        let modu = 1_000_000_007;
        let n = pizza.len();
        let m = pizza[0].len();
        for i in 0..n {
            let mut row = 0;
            for j in 0..m {
                if pizza[i].chars().nth(j).unwrap() == 'A' {
                    row += 1;
                }
                sum[i + 1][j + 1] = sum[i][j + 1] + row;
            }
        }
        dp[0][0][0] = 1;
        for i in 1..=k as usize {
            for x in 0..n {
                for y in 0..m {
                    if dp[i - 1][x][y] == 0 {
                        continue;
                    }
                    for a in x + 1..n {
                        if sum[a][m] - sum[x][m] == 0 {
                            break;
                        }
                        for b in y + 1..m {
                            if sum[a][b] - sum[x][b] == 0 {
                                break;
                            }
                            if sum[a][b] - sum[x][b] == sum[a][m] - sum[x][m] {
                                continue;
                            }
                            if sum[n][b] - sum[a][b] == 0 {
                                break;
                            }
                            if sum[n][m] - sum[a][m] == 0 {
                                break;
                            }
                            dp[i as usize][a][b] = dp[i as usize][a][b] % modu + pre[x][y] % modu;
                            dp[i as usize][a][b] = dp[i as usize][a][b] % modu;
                        }
                    }
                }
            }
            for x in 0..n {
                for y in 0..m {
                    pre[x][y] = pre[x][y] % modu + dp[i as usize][x][y] % modu;
                    pre[x][y] = pre[x][y] % modu;
                }
            }
        }
        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                ans = ans % modu + dp[k as usize][i][j] % modu;
                ans = ans % modu;
            }
        }
        return ans;
    }
}
