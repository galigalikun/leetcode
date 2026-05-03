fn main() {
    assert_eq!(Solution::calculate("1 + 1".to_string()), 2);
    assert_eq!(Solution::calculate(" 2-1 + 2 ".to_string()), 3);
    assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string()), 23);
}

struct Solution {}
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut result = 0;
        let mut sign = 1;
        let mut number = 0;
        let mut stack: Vec<(i32, i32)> = Vec::new();

        for ch in s.chars() {
            if ch.is_ascii_digit() {
                number = number * 10 + (ch as i32 - '0' as i32);
                continue;
            }

            if ch == '+' || ch == '-' {
                result += sign * number;
                number = 0;
                sign = if ch == '+' { 1 } else { -1 };
                continue;
            }

            if ch == '(' {
                stack.push((result, sign));
                result = 0;
                sign = 1;
                number = 0;
                continue;
            }

            if ch == ')' {
                result += sign * number;
                number = 0;

                if let Some((prev_result, prev_sign)) = stack.pop() {
                    result = prev_result + prev_sign * result;
                }
            }
        }

        result + sign * number
    }
}
