fn main() {
    assert_eq!(Solution::custom_sort_string("cba".to_string(), "abcd".to_string()), "cbad");
    assert_eq!(Solution::custom_sort_string("cbafg".to_string(), "abcd".to_string()), "cbad");
}

struct Solution{}
impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        return "".to_string();
    }
}
