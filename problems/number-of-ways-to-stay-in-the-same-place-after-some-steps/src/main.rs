fn main() {
    assert_eq!(Solution::num_ways(3, 2), 4);
    assert_eq!(Solution::num_ways(2, 4), 2);
    assert_eq!(Solution::num_ways(4, 2), 8);
}

struct Solution;
impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        let mut dp = vec![0; arr_len as usize];
        dp[0] = 1;
        for _ in 0..steps {
            let mut dp2 = vec![0; arr_len as usize];
            for i in 0..arr_len as usize {
                if dp[i] > 0 {
                    if i > 0 {
                        dp2[i - 1] = (dp2[i - 1] + dp[i]) % 1000000007;
                    }
                    dp2[i] = (dp2[i] + dp[i]) % 1000000007;
                    if i < arr_len as usize - 1 {
                        dp2[i + 1] = (dp2[i + 1] + dp[i]) % 1000000007;
                    }
                }
            }
            dp = dp2;
        }
        return dp[0];
    }
}
