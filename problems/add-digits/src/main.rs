fn main() {
    assert_eq!(Solution::add_digits(38), 2);
    assert_eq!(Solution::add_digits(0), 0);
}

struct Solution {}
impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if 10 > num && num >= 0 {
            return num;
        }
        return Solution::add_digits(
            num.to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .fold(0, |sum, a| sum + a) as i32,
        );
    }
}
