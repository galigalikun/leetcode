fn main() {
    assert_eq!(Solution::longest_valid_parentheses("()".to_string()), 2);
    assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
    assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
    assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
    assert_eq!(Solution::longest_valid_parentheses("()(()".to_string()), 2);
    assert_eq!(Solution::longest_valid_parentheses(")()())()()(".to_string()),4);
    assert_eq!(
        Solution::longest_valid_parentheses("(()(((()".to_string()),
        2
    );
}

pub struct Solution {}
// https://programmerstart.com/article/1255166812/
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut a:Vec<i32> = vec![-1];
        let mut n = 0;
        let mut ret: i32 = 0;
        for c in s.as_str().chars() {
            if c == '(' {
                a.push(n);
            } else {
                if let Some(&l) = a.last() {
                    if Some('(') == s.as_str().chars().nth(l as usize) {
                        a.pop();
                        if let Some(&l) = a.last() {
                            let new_len = n as i32 - l;
                            if new_len > ret {
                                ret = new_len;
                            }
                        }
                    } else {
                        a.push(n);
                    }
                } else {
                    a.push(n);
                }
            }
            n += 1;
        }
        return ret;
    }
}
