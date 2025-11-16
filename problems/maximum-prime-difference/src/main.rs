fn main() {
    assert_eq!(Solution::maximum_prime_difference(vec![4, 2, 9, 5, 3]), 3);
    assert_eq!(Solution::maximum_prime_difference(vec![4, 8, 2, 8]), 0);
    assert_eq!(Solution::maximum_prime_difference(vec![2, 2]), 1);
}

struct Solution;
impl Solution {
    pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
        fn is_prime(n: i32) -> bool {
            if n < 2 {
                return false;
            }
            for i in 2..=((n as f64).sqrt() as i32) {
                if n % i == 0 {
                    return false;
                }
            }
            true
        }

        let primes: Vec<i32> = nums.into_iter().filter(|&x| is_prime(x)).collect();
        if primes.len() < 2 {
            return 0;
        }
        let max_prime = *primes.iter().max().unwrap();
        let min_prime = *primes.iter().min().unwrap();
        max_prime - min_prime
    }
}
