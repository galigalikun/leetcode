fn main() {
    assert_eq!(
        Solution::reverse_prefix("abcdefd".to_string(), 'd'),
        "dcbaefd"
    );
    assert_eq!(
        Solution::reverse_prefix("xyxzxe".to_string(), 'z'),
        "zxyxxe"
    );
    assert_eq!(Solution::reverse_prefix("abcd".to_string(), 'z'), "abcd");
}

struct Solution;
impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        if let Some(pos) = word.find(ch) {
            let (first, last) = word.split_at(pos + 1);
            let reversed_first: String = first.chars().rev().collect();
            format!("{}{}", reversed_first, last)
        } else {
            word
        }
    }
}
