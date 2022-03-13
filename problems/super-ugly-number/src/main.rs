fn main() {
    assert_eq!(Solution::nth_super_ugly_number(12, vec![2, 7, 13, 19]), 32);
    assert_eq!(Solution::nth_super_ugly_number(1, vec![2, 3, 5]), 1);
}

struct Solution {}
// https://www.tutorialcup.com/interview/dynamic-programming/super-ugly-number.htm
impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let mut dp = vec![1; n as usize + 1];
        let k = primes.len();
        let mut pointer = vec![1; k];

        for i in 2..=n as usize {
            let mut mi = std::i32::MAX;
            for j in 0..k {
                let tmp = dp[pointer[j]] * primes[j];
                mi = std::cmp::min(mi, tmp);
            }
            dp[i] = mi;
            for j in 0..k {
                let tmp = dp[pointer[j]] * primes[j];
                if tmp == mi {
                    pointer[j] += 1;
                }
            }
        }

        return dp[n as usize];
    }
}
