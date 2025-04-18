fn main() {
    assert_eq!(Solution::max_nice_divisors(5), 6);
    assert_eq!(Solution::max_nice_divisors(8), 18);
    assert_eq!(Solution::max_nice_divisors(98), 351761402);
    assert_eq!(Solution::max_nice_divisors(88), 169179295);
    assert_eq!(Solution::max_nice_divisors(175769706), 529552630);
}

struct Solution;
impl Solution {
    pub fn max_nice_divisors(prime_factors: i32) -> i32 {
        return match prime_factors {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            _ => {
                let mut result = 1 as i64;
                let mut n = prime_factors as i64;
                while n > 4 {
                    result = (result * 3) % (1_000_000_007);
                    n -= 3;
                }
                (result * n % (1_000_000_007)) as i32
            }
        };
    }
}
