fn main() {
    assert_eq!(Solution::min_insertions("(()))".to_string()), 1);
    assert_eq!(Solution::min_insertions("())".to_string()), 0);
    assert_eq!(Solution::min_insertions("))())(".to_string()), 3);
    assert_eq!(Solution::min_insertions(")))))))".to_string()), 5);
}

struct Solution;
impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut i = 0;
        let mut count = 0;
        while i < s.len() {
            if s.chars().nth(i).unwrap() == '(' {
                stack.push('(');
                i += 1;
            } else {
                if i + 1 < s.len() && s.chars().nth(i + 1).unwrap() == ')' {
                    if stack.is_empty() {
                        count += 1;
                    } else {
                        stack.pop();
                    }
                    i += 2;
                } else {
                    if stack.is_empty() {
                        count += 1;
                    } else {
                        stack.pop();
                        count += 1;
                    }
                    i += 1;
                }
            }
        }
        count += stack.len() * 2;
        count as i32
    }
}
