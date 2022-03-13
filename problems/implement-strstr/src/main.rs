fn main() {
    assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2);
    assert_eq!(
        Solution::str_str("aaaaa".to_string(), "bba".to_string()),
        -1
    );
    assert_eq!(Solution::str_str("".to_string(), "".to_string()), 0);
}

struct Solution {}
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if let Some(p) = haystack.find(needle.as_str()) {
            return p as i32;
        }
        return -1;
    }
}
