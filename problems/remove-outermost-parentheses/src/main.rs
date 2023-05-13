fn main() {
    assert_eq!(Solution::remove_outer_parentheses("(()())(())".to_string()), "()()()");
    assert_eq!(Solution::remove_outer_parentheses("(()())(())(()(()))".to_string()), "()()()()(())");
    assert_eq!(Solution::remove_outer_parentheses("()()".to_string()), "");
}

struct Solution;
impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut ss = s.clone();
        let mut stack = Vec::new();
        let mut start = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                stack.push(c);
            } else {
                stack.pop();
            }
            if stack.is_empty() {
                ss.replace_range(start..=start, "");
                ss.replace_range(i - 1..=i - 1, "");
                start = i;
            }
        }
        return s;
    }
}
