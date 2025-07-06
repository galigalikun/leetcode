fn main() {
    assert_eq!(Solution::largest_odd_number("52".to_string()), "5");
    assert_eq!(Solution::largest_odd_number("4206".to_string()), "");
    assert_eq!(Solution::largest_odd_number("35427".to_string()), "35427");
}

struct Solution;
impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        return if let Some(i) = num
            .chars()
            .rev()
            .position(|c| c.to_digit(10).unwrap() % 2 == 1)
        {
            num[..num.len() - i].to_string()
        } else {
            String::new()
        };
    }
}
