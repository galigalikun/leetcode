fn main() {
    assert_eq!(Solution::thousand_separator(987), "987".to_string());
    assert_eq!(Solution::thousand_separator(1234), "1.234".to_string());
}

struct Solution;
impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        return n
            .to_string()
            .chars()
            .rev()
            .collect::<Vec<char>>()
            .chunks(3)
            .map(|x| x.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(".")
            .chars()
            .rev()
            .collect::<String>();
    }
}
