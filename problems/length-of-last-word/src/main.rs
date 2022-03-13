fn main() {
    assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
    assert_eq!(
        Solution::length_of_last_word("Today is a nice day".to_string()),
        3
    );
}

struct Solution {}
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        if let Some(&c) = s.split_whitespace().collect::<Vec<&str>>().last() {
            return c.len() as i32;
        }

        return 0;
    }
}
