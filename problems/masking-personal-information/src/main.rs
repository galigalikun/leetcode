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
        let mut ans = String::new();
        let mut a = false;
        let mut prev = None;
        for c in s.chars() {
            if c == '@' {
                a = true;
                ans = format!("{}{}{}", ans, "*".repeat(5), prev.unwrap());
            }
            if a {
                ans = format!("{}{}", ans, c.to_ascii_lowercase());
            } else if prev == None {
                ans = format!("{}{}", ans, c.to_ascii_lowercase());
            }
            prev = Some(c.to_ascii_lowercase());
        }
        return ans;
    }
}
