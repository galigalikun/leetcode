fn main() {
    assert_eq!(
        Solution::number_of_special_chars(String::from("aaAbcBC")),
        3
    );
    assert_eq!(Solution::number_of_special_chars(String::from("abc")), 0);
    assert_eq!(Solution::number_of_special_chars(String::from("AbBCab")), 3);
}

struct Solution;
impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        return word.chars().filter(|c| !c.is_alphanumeric()).count() as i32;
    }
}
