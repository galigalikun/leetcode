fn main() {
    assert_eq!(Solution::subdomain_visits(vec!["9001 discuss.leetcode.com".to_string()]), vec!["9001 leetcode.com","9001 discuss.leetcode.com","9001 com"]);
    assert_eq!(Solution::subdomain_visits(vec!["900 google.mail.com".to_string(), "50 yahoo.com".to_string(), "1 intel.mail.com".to_string(), "5 wiki.org".to_string()]), vec!["901 mail.com","50 yahoo.com","900 google.mail.com","5 wiki.org","5 org","1 intel.mail.com","951 com"]);
}

struct Solution{}
impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        return vec![];
    }
}
