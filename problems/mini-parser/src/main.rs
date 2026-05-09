fn main() {
    assert_eq!(
        Solution::deserialize("324".to_string()),
        NestedInteger::Int(324)
    );
    // assert_eq!(Solution::deserialize("[123,[456,[789]]]".to_string()), NestedInteger::List(vec![NestedInteger::Int(123), NestedInteger::List(vec![NestedInteger::Int(456), NestedInteger::List(vec![NestedInteger::Int(789)])])]));
}

struct Solution {}
#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}
impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        if !s.starts_with('[') {
            return NestedInteger::Int(s.parse::<i32>().unwrap());
        }

        let mut stack: Vec<NestedInteger> = Vec::new();
        let mut number = 0;
        let mut sign = 1;
        let mut in_number = false;

        for ch in s.chars() {
            match ch {
                '[' => stack.push(NestedInteger::List(Vec::new())),
                '-' => sign = -1,
                '0'..='9' => {
                    number = number * 10 + (ch as i32 - '0' as i32);
                    in_number = true;
                }
                ',' | ']' => {
                    if in_number {
                        if let Some(NestedInteger::List(items)) = stack.last_mut() {
                            items.push(NestedInteger::Int(sign * number));
                        }
                        number = 0;
                        sign = 1;
                        in_number = false;
                    }

                    if ch == ']' && stack.len() > 1 {
                        let nested = stack.pop().unwrap();
                        if let Some(NestedInteger::List(items)) = stack.last_mut() {
                            items.push(nested);
                        }
                    }
                }
                _ => {}
            }
        }

        stack.pop().unwrap()
    }
}
