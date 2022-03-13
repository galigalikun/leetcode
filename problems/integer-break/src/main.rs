fn main() {
    assert_eq!(Solution::integer_break(2), 1);
    assert_eq!(Solution::integer_break(10), 36);
    assert_eq!(Solution::integer_break(3), 2);
    assert_eq!(Solution::integer_break(5), 6);
}

struct Solution {}
// https://medium.com/@gepphkat/dynamic-programming-343-integer-break-4aa8dde3ee0f
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        dp[2] = 1;
        if n > 2 {
            dp[3] = 2;
        }
        for i in 4..=n {
            let mut greatest_max = -1;
            for j in 2..=i / 2 {
                let n1 = j;
                let n2 = i - j;
                let max_product1 = if dp[n1 as usize] > n1 {
                    dp[n1 as usize]
                } else {
                    n1
                };
                let max_product2 = if dp[n2 as usize] > n2 {
                    dp[n2 as usize]
                } else {
                    n2
                };
                let max = max_product1 * max_product2;
                greatest_max = std::cmp::max(greatest_max, max);
            }
            dp[i as usize] = greatest_max;
        }

        return dp[n as usize];
    }
}
