fn main() {
    assert_eq!(
        Solution::longest_decomposition("ghiabcdefhelloadamhelloabcdefghi".to_string()),
        7
    );
    assert_eq!(Solution::longest_decomposition("merchant".to_string()), 1);
    assert_eq!(
        Solution::longest_decomposition("antaprezatepzapreanta".to_string()),
        11
    );
    assert_eq!(Solution::longest_decomposition("a".to_string()), 1);
    assert_eq!(Solution::longest_decomposition("ab".to_string()), 1);
    assert_eq!(Solution::longest_decomposition("aaa".to_string()), 3);
    assert_eq!(Solution::longest_decomposition("aaaa".to_string()), 4);
    assert_eq!(Solution::longest_decomposition("abcba".to_string()), 5);
    assert_eq!(Solution::longest_decomposition("aabbaa".to_string()), 6);
}

struct Solution;
impl Solution {
    pub fn longest_decomposition(text: String) -> i32 {
        let s = text.as_str();
        if s.is_empty() {
            return 0;
        }

        let mut left = 0usize;
        let mut right = s.len();
        let mut result = 0;

        while left < right {
            let mut matched = false;

            for chunk_len in 1..=right - left {
                let left_end = left + chunk_len;
                let right_start = right - chunk_len;

                if left_end > right_start {
                    break;
                }

                if &s[left..left_end] == &s[right_start..right] {
                    left = left_end;
                    right = right_start;
                    result += 2;
                    matched = true;
                    break;
                }
            }

            if !matched {
                result += 1;
                break;
            }
        }

        result
    }
}
