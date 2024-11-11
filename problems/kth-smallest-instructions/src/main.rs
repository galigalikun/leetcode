fn main() {
    assert_eq!(Solution::kth_smallest_path(vec![2,3], 1), "HHHVV".to_string());
    assert_eq!(Solution::kth_smallest_path(vec![2,3], 2), "HHVHV".to_string());
    assert_eq!(Solution::kth_smallest_path(vec![2,3], 3), "HHVVH".to_string());
}

struct Solution;
impl Solution {
    pub fn kth_smallest_path(destination: Vec<i32>, k: i32) -> String {
        let mut dp = vec![vec![0; destination[1] as usize + 1]; destination[0] as usize + 1];
        for i in 0..=destination[0] as usize {
            dp[i][0] = 1;
        }
        for i in 0..=destination[1] as usize {
            dp[0][i] = 1;
        }
        for i in 1..=destination[0] as usize {
            for j in 1..=destination[1] as usize {
                dp[i][j] = dp[i-1][j] + dp[i][j-1];
            }
        }
        let mut x = destination[0] as usize;
        let mut y = destination[1] as usize;
        let mut k = k;
        let mut ans = "".to_string();
        while x > 0 && y > 0 {
            if k <= dp[x-1][y] {
                ans.push('H');
                x -= 1;
            } else {
                ans.push('V');
                k -= dp[x-1][y];
                y -= 1;
            }
        }
        while x > 0 {
            ans.push('H');
            x -= 1;
        }
        while y > 0 {
            ans.push('V');
            y -= 1;
        }
        ans
    }
}
