fn main() {
    assert_eq!(Solution::parse_bool_expr("&(|(f))".to_string()), false);
    assert_eq!(Solution::parse_bool_expr("|(f,f,f,t)".to_string()), true);
    assert_eq!(Solution::parse_bool_expr("!(&(f,t))".to_string()), true);
}

struct Solution;
impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let mut stack = vec![];
        for c in expression.chars() {
            if c == ')' {
                let mut temp = vec![];
                while let Some(top) = stack.pop() {
                    if top == '(' {
                        break;
                    }
                    temp.push(top);
                }
                let op = stack.pop().unwrap();
                if op == '&' {
                    stack.push(temp.iter().all(|&x| x == 't'));
                } else if op == '|' {
                    stack.push(temp.iter().any(|&x| x == 't'));
                } else if op == '!' {
                    stack.push(temp[0] == 'f');
                }
            } else if c != ',' {
                stack.push(c);
            }
        }
        return stack[0] == 't';
    }
}
