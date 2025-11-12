fn main() {
    assert_eq!(Solution::score_of_string("hello".to_string()), 13);
    assert_eq!(Solution::score_of_string("zaz".to_string()), 50);
}

struct Solution;
impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        return s.chars().map(|c| (c as u8 - b'A' + 1) as i32).sum();
    }
}
