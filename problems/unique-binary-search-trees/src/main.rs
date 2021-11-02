fn main() {
    assert_eq!(Solution::num_trees(3), 5);
    assert_eq!(Solution::num_trees(1), 1);
    assert_eq!(Solution::num_trees(19), 1767263190);
}

pub struct Solution {}
// https://www.geeksforgeeks.org/number-of-unique-bst-with-a-given-key-dynamic-programming/
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..=n as usize {
            for j in 1..=i {
                dp[i] = dp[i] + (dp[i - j] * dp[j - 1]);
            }
        }
        return dp[n as usize];
    }
}
