fn main() {
    assert_eq!(
        Solution::generate_parenthesis(3),
        vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
    );
    assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
}

pub struct Solution {}
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut s = "".to_string();
        for _ in 0..n {
            s.push_str("()");
        }
        /*
        ss.push(n).push_str("(").push(n).push_str(")");
        ss.push(n).push_str("((").push(n).push_str(")");
        */
        result.push(s);
        return result;
    }
}
