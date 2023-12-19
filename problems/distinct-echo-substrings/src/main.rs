fn main() {
    assert_eq!(
        Solution::distinct_echo_substrings("abcabcabc".to_string()),
        3
    );
    assert_eq!(
        Solution::distinct_echo_substrings("leetcodeleetcode".to_string()),
        2
    );
}

struct Solution;
impl Solution {
    pub fn distinct_echo_substrings(text: String) -> i32 {
        let mut set = std::collections::HashSet::new();
        let chars: Vec<char> = text.chars().collect();
        for i in 0..chars.len() {
            for j in (i + 1)..chars.len() {
                if j - i > chars.len() / 2 {
                    break;
                }
                let mut is_echo = true;
                for k in 0..(j - i) {
                    if chars[i + k] != chars[j + k] {
                        is_echo = false;
                        break;
                    }
                }
                if is_echo {
                    set.insert(&text[i..j]);
                }
            }
        }
        return set.len() as i32;
    }
}
