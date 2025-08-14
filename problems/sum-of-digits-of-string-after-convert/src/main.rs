fn main() {
    assert_eq!(Solution::get_lucky("iiii".to_string(), 1), 36);
    assert_eq!(Solution::get_lucky("leetcode".to_string(), 2), 6);
    assert_eq!(Solution::get_lucky("zbax".to_string(), 2), 8);
}

struct Solution;
impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut sum = 0;
        for c in s.chars() {
            sum += (c as u8 - b'a' + 1) as i32;
        }
        let mut result = sum;
        for _ in 0..k {
            result = result
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .sum();
        }
        result
    }
}
