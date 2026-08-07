fn main() {
    assert_eq!(
        Solution::mask_pii("LeetCode@LeetCode.com".to_string()),
        "l*****e@leetcode.com"
    );
    assert_eq!(
        Solution::mask_pii("AB@qq.com".to_string()),
        "a*****b@qq.com"
    );
    assert_eq!(Solution::mask_pii("1(234)567-890".to_string()), "***-***-7890");
}

struct Solution {}
impl Solution {
    pub fn mask_pii(s: String) -> String {
        if s.contains('@') {
            let lower = s.to_ascii_lowercase();
            let (name, domain) = lower
                .split_once('@')
                .expect("email input must contain @");
            let first = name.chars().next().expect("email name must not be empty");
            let last = name.chars().last().expect("email name must not be empty");
            format!("{first}*****{last}@{domain}")
        } else {
            let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
            let local = &digits[digits.len() - 4..];
            let country_len = digits.len() - 10;

            if country_len == 0 {
                format!("***-***-{local}")
            } else {
                format!("+{}-***-***-{local}", "*".repeat(country_len))
            }
        }
    }
}
