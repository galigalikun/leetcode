fn main() {
    assert_eq!(Solution::score_of_parentheses("()".to_string()), 1);
    assert_eq!(Solution::score_of_parentheses("(())".to_string()), 2);
    assert_eq!(Solution::score_of_parentheses("()()".to_string()), 2);
}

struct Solution;
impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut depth: usize = 0;
        let mut score = 0;

        for i in 0..bytes.len() {
            if bytes[i] == b'(' {
                depth += 1;
            } else {
                depth -= 1;
                if i > 0 && bytes[i - 1] == b'(' {
                    score += 1 << depth;
                }
            }
        }

        score
    }
}
