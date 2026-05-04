fn main() {
    assert_eq!(Solution::calculate("3+2*2".to_string()), 7);
    assert_eq!(Solution::calculate(" 3/2 ".to_string()), 1);
    assert_eq!(Solution::calculate(" 3+5 / 2 ".to_string()), 5);
}

struct Solution;
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut result: i64 = 0;
        let mut last_term: i64 = 0;
        let mut number: i64 = 0;
        let mut operator = b'+';

        for (i, &ch) in bytes.iter().enumerate() {
            if ch.is_ascii_digit() {
                number = number * 10 + i64::from(ch - b'0');
            }

            if (!ch.is_ascii_digit() && ch != b' ') || i == bytes.len() - 1 {
                match operator {
                    b'+' => {
                        result += last_term;
                        last_term = number;
                    }
                    b'-' => {
                        result += last_term;
                        last_term = -number;
                    }
                    b'*' => {
                        last_term *= number;
                    }
                    b'/' => {
                        last_term /= number;
                    }
                    _ => {}
                }
                operator = ch;
                number = 0;
            }
        }

        (result + last_term) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluates_examples() {
        assert_eq!(Solution::calculate("3+2*2".to_string()), 7);
        assert_eq!(Solution::calculate(" 3/2 ".to_string()), 1);
        assert_eq!(Solution::calculate(" 3+5 / 2 ".to_string()), 5);
    }

    #[test]
    fn handles_mixed_operations() {
        assert_eq!(Solution::calculate("14-3/2".to_string()), 13);
        assert_eq!(Solution::calculate("42".to_string()), 42);
        assert_eq!(Solution::calculate("1-1+1".to_string()), 1);
    }
}
