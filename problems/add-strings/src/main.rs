fn main() {
    assert_eq!(
        Solution::add_strings("11".to_string(), "123".to_string()),
        "134"
    );
    assert_eq!(
        Solution::add_strings("456".to_string(), "77".to_string()),
        "533"
    );
    assert_eq!(Solution::add_strings("0".to_string(), "0".to_string()), "0");
    assert_eq!(
        Solution::add_strings("1".to_string(), "9".to_string()),
        "10"
    );
    assert_eq!(
        Solution::add_strings("6994".to_string(), "36".to_string()),
        "7030"
    );
}

struct Solution {}
// https://cheonhyangzhang.gitbooks.io/leetcode-solutions/content/415-add-strings.html
impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut result = "".to_string();
        let mut carry = 0;
        for i in 0..if num1.len() > num2.len() {
            num1.len()
        } else {
            num2.len()
        } {
            match (num1.chars().rev().nth(i), num2.chars().rev().nth(i)) {
                (Some(n1), Some(n2)) => {
                    let num = n1 as u8 - 48 + n2 as u8 - 48 + carry;
                    carry = num / 10;
                    let digit = num % 10;
                    result = format!("{}{}", digit, result);
                }
                (Some(n1), None) => {
                    let num = n1 as u8 - 48 + carry;
                    carry = num / 10;
                    let digit = num % 10;
                    result = format!("{}{}", digit, result);
                }
                (None, Some(n2)) => {
                    let num = n2 as u8 - 48 + carry;
                    carry = num / 10;
                    let digit = num % 10;
                    result = format!("{}{}", digit, result);
                }
                (None, None) => {
                    if carry > 0 {
                        result = format!("{}{}", carry, result);
                        carry = 0;
                    }
                }
            }
        }
        if carry > 0 {
            result = format!("{}{}", carry, result);
        }
        return result;
    }
}
