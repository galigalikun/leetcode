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
}

struct Solution;
impl Solution {
    pub fn longest_decomposition(text: String) -> i32 {
        let mut result = 0;
        let mut left = 0;
        let mut right = text.len() - 1;
        let mut left_str = String::new();
        let mut right_str = String::new();
        while left < right {
            left_str.push(text.chars().nth(left).unwrap());
            right_str.insert(0, text.chars().nth(right).unwrap());
            if left_str == right_str {
                result += 2;
                left_str.clear();
                right_str.clear();
            }
            left += 1;
            right -= 1;
        }
        if left == right && left_str == right_str {
            result += 1;
        }
        return result;
    }
}
