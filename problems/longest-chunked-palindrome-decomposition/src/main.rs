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
}

struct Solution;
impl Solution {
    pub fn longest_decomposition(text: String) -> i32 {
        if text.is_empty() {
            return 0;
        }

        let chars: Vec<char> = text.chars().collect();
        let mut result = 0;
        let mut left = 0usize;
        let mut right = chars.len() - 1;
        let mut left_str = String::new();
        let mut right_str = String::new();

        while left < right {
            left_str.push(chars[left]);
            right_str.insert(0, chars[right]);

            if left_str == right_str {
                result += 2;
                left_str.clear();
                right_str.clear();
            }

            left += 1;
            right -= 1;
        }

        if left == right {
            result += 1;
        } else if !left_str.is_empty() || !right_str.is_empty() {
            result += 1;
        }

        result
    }
}
