fn main() {
    assert_eq!(
        Solution::num_unique_emails(vec![
            "test.email+alex@leetcode.com".to_string(),
            "test.e.mail+bob.cathy@leetcode.com".to_string(),
            "testemail+david@lee.tcode.com".to_string()
        ]),
        2
    );
    assert_eq!(
        Solution::num_unique_emails(vec![
            "a@leetcode.com".to_string(),
            "b@leetcode.com".to_string(),
            "c@leetcode.com".to_string()
        ]),
        3
    );
}

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut ans = HashMap::new();
        for email in emails {
            let mut a = email.split("@");
            ans.insert(
                format!(
                    "{}@{}",
                    a.next()
                        .unwrap()
                        .replace(".", "")
                        .split("+")
                        .nth(0)
                        .unwrap(),
                    a.next().unwrap()
                ),
                1,
            );
        }
        return ans.len() as i32;
    }
}
