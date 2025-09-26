fn main() {
    assert_eq!(Solution::number_of_good_subsets(vec![1, 2, 3, 4]), 6);
    assert_eq!(Solution::number_of_good_subsets(vec![4, 2, 3, 15]), 5);
}

struct Solution;
impl Solution {
    pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
        let mod_num = 1_000_000_007;
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        let mut counts = vec![0; 1 << primes.len()];
        let mut ones = 0;
        for &num in &nums {
            if num == 1 {
                ones += 1;
                continue;
            }
            let mut subset = 0;
            let mut n = num;
            let mut valid = true;
            for (i, &prime) in primes.iter().enumerate() {
                let mut count = 0;
                while n % prime == 0 {
                    n /= prime;
                    count += 1;
                }
                if count > 1 {
                    valid = false;
                    break;
                }
                if count == 1 {
                    subset |= 1 << i;
                }
            }
            if valid && n == 1 {
                counts[subset] += 1;
            }
        }
        let mut dp = vec![0; 1 << primes.len()];
        dp[0] = 1;
        for (mask, &count) in counts.iter().enumerate() {
            if count == 0 {
                continue;
            }
            for prev in (0..(1 << primes.len())).rev() {
                if prev & mask == 0 {
                    dp[prev | mask] = (dp[prev | mask] + dp[prev] * count) % mod_num;
                }
            }
        }
        let total_subsets: i32 = dp.iter().skip(1).sum::<i32>() % mod_num;
        let pow2_ones = (1..=ones).fold(1, |acc, _| (acc * 2) % mod_num);
        (total_subsets * pow2_ones) % mod_num
    }
}
