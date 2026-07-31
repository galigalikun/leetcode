fn main() {
    assert_eq!(Solution::ambiguous_coordinates("(123)".to_string()), vec!["(1, 2.3)","(1, 23)","(1.2, 3)","(12, 3)"]);
    assert_eq!(Solution::ambiguous_coordinates("(0123)".to_string()), vec!["(0, 1.23)","(0, 12.3)","(0, 123)","(0.1, 2.3)","(0.1, 23)","(0.12, 3)"]);
    assert_eq!(Solution::ambiguous_coordinates("(00011)".to_string()), vec!["(0, 0.011)","(0.001, 1)"]);
}

struct Solution{}
impl Solution {
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let digits = &s[1..s.len() - 1];
        let mut coordinates = Vec::new();

        for split_idx in 1..digits.len() {
            let left = &digits[..split_idx];
            let right = &digits[split_idx..];
            let left_candidates = Self::valid_numbers(left);
            let right_candidates = Self::valid_numbers(right);

            for left_value in &left_candidates {
                for right_value in &right_candidates {
                    coordinates.push(format!("({}, {})", left_value, right_value));
                }
            }
        }

        coordinates
    }

    fn valid_numbers(part: &str) -> Vec<String> {
        if part.len() == 1 {
            return vec![part.to_string()];
        }

        let bytes = part.as_bytes();
        let starts_with_zero = bytes[0] == b'0';
        let ends_with_zero = bytes[bytes.len() - 1] == b'0';

        if starts_with_zero && ends_with_zero {
            return Vec::new();
        }

        if starts_with_zero {
            return vec![format!("0.{}", &part[1..])];
        }

        if ends_with_zero {
            return vec![part.to_string()];
        }

        let mut values = Vec::new();
        for decimal_idx in 1..part.len() {
            values.push(format!("{}.{}", &part[..decimal_idx], &part[decimal_idx..]));
        }
        values.push(part.to_string());
        values
    }
}
