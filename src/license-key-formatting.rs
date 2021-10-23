fn main() {
    assert_eq!(
        Solution::license_key_formatting("5F3Z-2e-9-w".to_string(), 4),
        "5F3Z-2E9W"
    );
    assert_eq!(
        Solution::license_key_formatting("2-5g-3-J".to_string(), 2),
        "2-5G-3J"
    );
    assert_eq!(
        Solution::license_key_formatting("--a-a-a-a--".to_string(), 2),
        "AA-AA"
    );
}

pub struct Solution {}
impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut result = vec![];
        let mut n = 0;
        for c in s.chars().rev() {
            if n == k {
                result.push('-');
                n = 0;
            }
            if c != '-' {
                result.push(c.to_ascii_uppercase());
                n += 1;
            }
        }
        if Some(&'-') == result.last() {
            result.remove(result.len()-1);
        }
        return result.iter().rev().collect::<String>();
    }
}
