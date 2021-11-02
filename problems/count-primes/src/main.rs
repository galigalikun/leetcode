fn main() {
    assert_eq!(Solution::count_primes(10), 4);

    assert_eq!(Solution::count_primes(0), 0);

    assert_eq!(Solution::count_primes(1), 0);

    assert_eq!(Solution::count_primes(2), 0);
}

pub struct Solution {}
impl Solution {
    fn is_primes(n: i32) -> bool {
        if n == 2 {
            return true;
        } else if n == 3 {
            return true;
        } else if n % 2 == 1 {
            for x in 2..=(n as f64).sqrt() as i32 {
                if n % x == 0 {
                    return false;
                }
            }
            return true;
        }
        return false;
    }
    pub fn count_primes(n: i32) -> i32 {
        return (2..n).filter(|&x| Solution::is_primes(x)).count() as i32;
    }
}
