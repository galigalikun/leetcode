fn main() {
    assert_eq!(Solution::remove_outer_parentheses("(()())(())".to_string()), "()()()");
    assert_eq!(Solution::remove_outer_parentheses("(()())(())(()(()))".to_string()), "()()()()(())");
    assert_eq!(Solution::remove_outer_parentheses("()()".to_string()), "");
}

struct Solution;
impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut depth = 0;
        let mut result = String::with_capacity(s.len());

        for c in s.chars() {
            if c == '(' {
                if depth > 0 {
                    result.push(c);
                }
                depth += 1;
            } else {
                depth -= 1;
                if depth > 0 {
                    result.push(c);
                }
            }
        }

        result
    }
}
