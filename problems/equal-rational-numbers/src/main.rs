fn main() {
    assert_eq!(Solution::is_rational_equal("0.(52)".to_string(), "0.5(25)".to_string()), true);
    assert_eq!(Solution::is_rational_equal("0.1666(6)".to_string(), "0.166(66)".to_string()), true);
    assert_eq!(Solution::is_rational_equal("0.9(9)".to_string(), "1.".to_string()), true);
}

struct Solution;
impl Solution {
    pub fn is_rational_equal(s: String, t: String) -> bool {
        Self::to_fraction(&s) == Self::to_fraction(&t)
    }

    fn to_fraction(x: &str) -> (i128, i128) {
        let (int_part, frac_part) = x.split_once('.').unwrap_or((x, ""));
        let integer = int_part.parse::<i128>().unwrap();

        let (non_repeating, repeating) = if let Some((nr, rep_with_paren)) = frac_part.split_once('(') {
            (nr, rep_with_paren.strip_suffix(')').unwrap())
        } else {
            (frac_part, "")
        };

        let m = non_repeating.len() as u32;
        let n = repeating.len() as u32;
        let non_rep = if non_repeating.is_empty() {
            0
        } else {
            non_repeating.parse::<i128>().unwrap()
        };

        let (mut num, mut den) = if n == 0 {
            let den = Self::pow10(m);
            (integer * den + non_rep, den)
        } else {
            let rep = repeating.parse::<i128>().unwrap();
            let pow_m = Self::pow10(m);
            let pow_n = Self::pow10(n);
            let den = pow_m * (pow_n - 1);
            let num = integer * den + non_rep * (pow_n - 1) + rep;
            (num, den)
        };

        let g = Self::gcd(num, den);
        num /= g;
        den /= g;
        (num, den)
    }

    fn pow10(exp: u32) -> i128 {
        let mut v = 1i128;
        for _ in 0..exp {
            v *= 10;
        }
        v
    }

    fn gcd(mut a: i128, mut b: i128) -> i128 {
        a = a.abs();
        b = b.abs();
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }
}
