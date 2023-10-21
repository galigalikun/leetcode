fn main() {
    assert_eq!(
        Solution::min_remove_to_make_valid("lee(t(c)o)de)".to_string()),
        "lee(t(c)o)de"
    );
    assert_eq!(
        Solution::min_remove_to_make_valid("a)b(c)d".to_string()),
        "ab(c)d"
    );
    assert_eq!(Solution::min_remove_to_make_valid("))((".to_string()), "");
}

struct Solution;
impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut stack: Vec<usize> = Vec::new();
        let mut s_chars: Vec<char> = s.chars().collect();
        for i in 0..s_chars.len() {
            if s_chars[i] == '(' {
                stack.push(i);
            } else if s_chars[i] == ')' {
                if stack.len() > 0 {
                    stack.pop();
                } else {
                    s_chars[i] = ' ';
                }
            }
        }
        while stack.len() > 0 {
            s_chars[stack.pop().unwrap()] = ' ';
        }
        return s_chars.into_iter().filter(|&c| c != ' ').collect();
    }
}
