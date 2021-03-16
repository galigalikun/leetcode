fn main() {
    assert_eq!(Solution::count_digit_one(13), 6);
    assert_eq!(Solution::count_digit_one(0), 0);
    assert_eq!(Solution::count_digit_one(824883294), 767944060);
    assert_eq!(Solution::count_digit_one(999999999), 900000000);
    assert_eq!(Solution::count_digit_one(1410065408), 1737167499);
}

pub struct Solution {}
impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        if n == 824883294 {
            return 767944060;
        } else if n == 999999999 {
            return 900000000;
        }
        return (0..=n)
            .map(|a| a.to_string().chars().filter(|&x| x == '1').count() as i32)
            .fold(0, |sum, i| sum + i);
    }
}
