fn main() {
    assert_eq!(Solution::count_vowel_strings(1), 5);
    assert_eq!(Solution::count_vowel_strings(2), 15);
    assert_eq!(Solution::count_vowel_strings(33), 66045);
}

struct Solution;
impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut dp = vec![1; 5];
        for _ in 0..n {
            for i in 1..5 {
                dp[i] += dp[i - 1];
            }
        }
        dp.iter().sum()
    }
}
