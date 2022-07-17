fn main() {
    assert_eq!(Solution::count_prime_set_bits(6, 10), 4);
    assert_eq!(Solution::count_prime_set_bits(10, 15), 5);
}

struct Solution {}
impl Solution {
    fn is_primes(n: i32) -> bool {
        if n == 1 {
            return false;
        } else if n == 2 {
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
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        return (left..=right)
            .filter(|n| {
                Solution::is_primes(
                    format!("{0:b}", &n).chars().filter(|c| c == &'1').count() as i32
                )
            })
            .count() as i32;
    }
}
