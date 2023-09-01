fn main() {
    assert_eq!(Solution::num_prime_arrangements(5), 12);
    assert_eq!(Solution::num_prime_arrangements(100), 682289015);
}

struct Solution;
impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        let mut primes = vec![true; n as usize + 1];
        primes[0] = false;
        primes[1] = false;
        let mut i = 2;
        while i * i <= n {
            if primes[i as usize] {
                let mut j = i * i;
                while j <= n {
                    primes[j as usize] = false;
                    j += i;
                }
            }
            i += 1;
        }
        let mut prime_count = 0;
        for i in 0..=n {
            if primes[i as usize] {
                prime_count += 1;
            }
        }
        let mut ans: i128 = 1;
        for i in 1..=prime_count {
            ans = (ans * i as i128) % 1_000_000_007;
        }
        for i in 1..=(n - prime_count) {
            ans = (ans * i as i128) % 1_000_000_007;
        }
        return ans as i32;
    }
}
