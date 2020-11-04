fn main() {
    // assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
    // assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
    // assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
    // assert_eq!(Solution::longest_valid_parentheses("()(()".to_string()), 2);
    // assert_eq!(Solution::longest_valid_parentheses(")()())()()(".to_string()),4);
    assert_eq!(
        Solution::longest_valid_parentheses("(()(((()".to_string()),
        2
    );
}

pub struct Solution {}
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut a = Vec::new();
        let mut b = 0;
        let mut n = 0;
        let mut ret: i32 = 0;
        for c in s.as_str().chars() {
            match c {
                '(' => {
                    a.push(c);
                    n += 1;
                }
                ')' => {
                    if a.len() > 0 {
                        a.pop();
                        n += 1;
                        if a.len() == 0 {
                            b = n + b;
                            if b > ret {
                                ret = b;
                            }
                            n = 0;
                        }
                    } else {
                        if n > ret {
                            ret = n;
                        }
                        n = 0;
                        b = 0;
                    }
                }
                _ => {}
            }
        }
        if n > 0 {
            println!("debug {} {}", n, a.len());
            n -= a.len() as i32;
            if n > ret {
                ret = n;
            }
        }
        return ret;
    }
}
