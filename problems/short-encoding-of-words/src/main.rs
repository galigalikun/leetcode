fn main() {
    assert_eq!(Solution::minimum_length_encoding(vec!["time".to_string(), "me".to_string(), "bell".to_string()]), 10);
    assert_eq!(Solution::minimum_length_encoding(vec!["t".to_string()]), 2);
}

struct Solution{}
impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        return -1;
    }
}
