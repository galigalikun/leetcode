fn main() {
    assert_eq!(Solution::monotone_increasing_digits(1), 1);
    assert_eq!(Solution::monotone_increasing_digits(10), 9);
    assert_eq!(Solution::monotone_increasing_digits(1234), 1234);
    assert_eq!(Solution::monotone_increasing_digits(332), 299);
}

struct Solution{}
impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut digits: Vec<u8> = n.to_string().bytes().collect();
        let mut marker = digits.len();

        for index in (1..digits.len()).rev() {
            if digits[index - 1] > digits[index] {
                digits[index - 1] -= 1;
                marker = index;
            }
        }

        for digit in digits.iter_mut().skip(marker) {
            *digit = b'9';
        }

        digits
            .into_iter()
            .fold(0, |accumulator, digit| accumulator * 10 + (digit - b'0') as i32)
    }
}
