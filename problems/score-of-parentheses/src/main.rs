fn main() {
    assert_eq!(Solution::score_of_parentheses("()".to_string()), 1);
    assert_eq!(Solution::score_of_parentheses("(())".to_string()), 2);
    assert_eq!(Solution::score_of_parentheses("()()".to_string()), 2);
}

struct Solution;
impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        return 0;
    }
}
