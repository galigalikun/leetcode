fn main() {
    assert_eq!(Solution::is_valid("()".to_string()), true);
    assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    assert_eq!(Solution::is_valid("(]".to_string()), false);
    assert_eq!(Solution::is_valid("([)]".to_string()), false);
    assert_eq!(Solution::is_valid("{[]}".to_string()), true);
    assert_eq!(Solution::is_valid("[".to_string()), false);
    assert_eq!(Solution::is_valid("((".to_string()), false);
    assert_eq!(Solution::is_valid("))".to_string()), false);
}

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 == 1 {
            return false;
        }
        let mut prev = Vec::new();
        for c in s.as_str().chars() {
            match c {
                '(' => {
                    prev.push(c);
                }
                ')' => {
                    if let Some(p) = prev.pop() {
                        if p != '(' {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                '{' => {
                    prev.push(c);
                }
                '}' => {
                    if let Some(p) = prev.pop() {
                        if p != '{' {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                '[' => {
                    prev.push(c);
                }
                ']' => {
                    if let Some(p) = prev.pop() {
                        if p != '[' {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => {}
            }
        }

        return prev.len() == 0;
    }
}
