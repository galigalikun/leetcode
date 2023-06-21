fn main() {
    assert_eq!(
        Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string()),
        "ABC"
    );
    assert_eq!(
        Solution::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()),
        "AB"
    );
    assert_eq!(
        Solution::gcd_of_strings("LEET".to_string(), "CODE".to_string()),
        ""
    );
    assert_eq!(
        Solution::gcd_of_strings("EFGABC".to_string(), "ABC".to_string()),
        ""
    );
}

struct Solution;
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let mut str1 = str1;
        let mut str2 = str2;
        while str1 != str2 {
            if str1.len() > str2.len() {
                str1 = str1[str2.len()..].to_string();
            } else if str1.len() < str2.len() {
                str2 = str2[str1.len()..].to_string();
            } else {
                return String::from("");
            }
        }

        return str1;
    }
}
