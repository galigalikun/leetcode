fn main() {
    assert_eq!(Solution::parse_bool_expr("&(|(f))".to_string()), false);
    assert_eq!(Solution::parse_bool_expr("|(f,f,f,t)".to_string()), true);
    assert_eq!(Solution::parse_bool_expr("!(&(f,t))".to_string()), true);
}

struct Solution;
impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let mut stack: Vec<char> = vec![];
        for c in expression.chars() {
            if c == ')' {
                let mut temp = vec![];
                while let Some(top) = stack.pop() {
                    if top == '(' {
                        break;
                    }
                    if top != ',' {
                        temp.push(top);
                    }
                }
                let op = stack.pop().unwrap();
                let result = match op {
                    '&' => temp.iter().all(|&x| x == 't'),
                    '|' => temp.iter().any(|&x| x == 't'),
                    '!' => temp[0] == 'f',
                    _ => unreachable!(),
                };
                stack.push(if result { 't' } else { 'f' });
            } else if c != ',' {
                stack.push(c);
            }
        }
        stack[0] == 't'
    }
}
