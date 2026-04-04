fn main() {
    assert_eq!(
        Solution::generate_string("TFTF".to_string(), "ab".to_string()),
        "ababa".to_string()
    );
    assert_eq!(
        Solution::generate_string("TFTF".to_string(), "abc".to_string()),
        "".to_string()
    );
    assert_eq!(
        Solution::generate_string("F".to_string(), "d".to_string()),
        "a".to_string()
    );
}

struct Solution;
impl Solution {
    pub fn generate_string(str1: String, str2: String) -> String {
        let mut str1 = str1;
        let mut str2 = str2;
        let pairs: Vec<_> = str1.chars().zip(str2.chars()).collect();
        for (c1, c2) in pairs {
            if c1 == 'T' && c2 == 'a' {
                str1.push('a');
            } else if c1 == 'F' && c2 == 'b' {
                str2.push('b');
            }
        }
        return if str1.len() == str2.len() {
            String::new()
        } else {
            if str1.len() >= str2.len() { str1 } else { str2 }
        };
    }
}
