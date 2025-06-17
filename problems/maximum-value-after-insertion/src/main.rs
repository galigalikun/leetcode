fn main() {
    assert_eq!(Solution::max_value("99".to_string(), 9), "999".to_string());
    assert_eq!(
        Solution::max_value("-13".to_string(), 2),
        "-123".to_string()
    );
}

struct Solution;
impl Solution {
    pub fn max_value(n: String, x: i32) -> String {
        return if n.starts_with('-') {
            let mut chars: Vec<char> = n.chars().collect();
            let mut inserted = false;
            for i in 1..chars.len() {
                if chars[i].to_digit(10).unwrap() > x as u32 {
                    chars.insert(i, char::from_digit(x as u32, 10).unwrap());
                    inserted = true;
                    break;
                }
            }
            if !inserted {
                chars.push(char::from_digit(x as u32, 10).unwrap());
            }
            chars.into_iter().collect()
        } else {
            let mut chars: Vec<char> = n.chars().collect();
            let mut inserted = false;
            for i in 0..chars.len() {
                if chars[i].to_digit(10).unwrap() < x as u32 {
                    chars.insert(i, char::from_digit(x as u32, 10).unwrap());
                    inserted = true;
                    break;
                }
            }
            if !inserted {
                chars.push(char::from_digit(x as u32, 10).unwrap());
            }
            chars.into_iter().collect()
        };
    }
}
