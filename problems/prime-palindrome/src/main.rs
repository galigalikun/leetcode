fn main() {
    assert_eq!(Solution::prime_palindrome(6), 7);
    assert_eq!(Solution::prime_palindrome(8), 11);
    assert_eq!(Solution::prime_palindrome(13), 101);
}

struct Solution;
impl Solution {
    pub fn prime_palindrome(n: i32) -> i32 {
        if n <= 2 {
            return 2;
        }
        if n <= 3 {
            return 3;
        }
        if n <= 5 {
            return 5;
        }
        if n <= 7 {
            return 7;
        }
        if n <= 11 {
            return 11;
        }

        let mut half = 1;
        while half < 100_000 {
            let candidate = Self::build_odd_palindrome(half);
            if candidate >= n && Self::is_prime(candidate) {
                return candidate;
            }
            half += 1;
        }

        unreachable!("problem guarantees an answer in range");
    }

    fn build_odd_palindrome(prefix: i32) -> i32 {
        let mut value = prefix;
        let mut x = prefix / 10;

        while x > 0 {
            value = value * 10 + (x % 10);
            x /= 10;
        }

        value
    }

    fn is_prime(x: i32) -> bool {
        if x < 2 {
            return false;
        }
        if x % 2 == 0 {
            return x == 2;
        }

        let mut d = 3;
        while d <= x / d {
            if x % d == 0 {
                return false;
            }
            d += 2;
        }

        true
    }
}
