fn main() {
    assert_eq!(Solution::count_routes(vec![2, 3, 6, 8, 4], 1, 3, 5), 4);
    assert_eq!(Solution::count_routes(vec![4,3,1], 1, 0, 6), 5);
    assert_eq!(Solution::count_routes(vec![5,2,1], 0, 2, 3), 0);
}

struct Solution;
impl Solution {
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        let n = locations.len();
        let mut dp = vec![vec![-1; n]; fuel as usize + 1];
        let mod_num = 1_000_000_007;
        let mut dfs = |i: usize, f: i32| -> i32 {
            if dp[f as usize][i] != -1 {
                return dp[f as usize][i];
            }
            let mut ans = 0;
            if i as i32 == finish {
                ans += 1;
            }
            for j in 0..n {
                if i != j {
                    let d = (locations[i] - locations[j]).abs();
                    if f >= d {
                        ans += dfs(j, f - d);
                        ans %= mod_num;
                    }
                }
            }
            dp[f as usize][i] = ans;
            ans
        };
        dfs(start as usize, fuel)
    }
}
