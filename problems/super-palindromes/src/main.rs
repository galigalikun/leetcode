fn main() {
    assert_eq!(Solution::superpalindromes_in_range("4".to_string(), "1000".to_string()), 4);
    assert_eq!(Solution::superpalindromes_in_range("1".to_string(), "2".to_string()), 1);
}

struct Solution;
impl Solution {
    pub fn superpalindromes_in_range(left: String, right: String) -> i32 {
        let left = left.parse::<u128>().unwrap();
        let right = right.parse::<u128>().unwrap();
        let upper = integer_sqrt_floor(right);

        let mut count = 0;
        let mut prefix = 1u128;

        loop {
            let odd = build_palindrome(prefix, true);
            if odd > upper {
                break;
            }
            count += is_super_palindrome(odd, left, right) as i32;

            let even = build_palindrome(prefix, false);
            if even <= upper {
                count += is_super_palindrome(even, left, right) as i32;
            }

            prefix += 1;
        }

        count
    }
}

fn is_super_palindrome(root: u128, left: u128, right: u128) -> bool {
    let square = root * root;
    square >= left && square <= right && is_palindrome(square)
}

fn build_palindrome(prefix: u128, odd: bool) -> u128 {
    let mut result = prefix;
    let mut suffix = if odd { prefix / 10 } else { prefix };

    while suffix > 0 {
        result = result * 10 + suffix % 10;
        suffix /= 10;
    }

    result
}

fn is_palindrome(value: u128) -> bool {
    let digits = value.to_string();
    digits.chars().eq(digits.chars().rev())
}

fn integer_sqrt_floor(value: u128) -> u128 {
    let mut low = 0u128;
    let mut high = value;
    let mut answer = 0u128;

    while low <= high {
        let mid = (low + high) / 2;
        let square = mid * mid;

        if square <= value {
            answer = mid;
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    answer
}
