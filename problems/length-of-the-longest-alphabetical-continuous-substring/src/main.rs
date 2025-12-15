fn main() {
    assert_eq!(
        Solution::longest_continuous_substring("abacaba".to_string()),
        2
    );
    assert_eq!(
        Solution::longest_continuous_substring("abcde".to_string()),
        5
    );
}

struct Solution;
impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        return s
            .as_bytes()
            .windows(2)
            .fold((1, 1), |(max_len, curr_len), window| {
                if window[1] == window[0] + 1 {
                    (max_len.max(curr_len + 1), curr_len + 1)
                } else {
                    (max_len, 1)
                }
            })
            .0;
    }
}
