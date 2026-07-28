use std::collections::HashMap;

fn main() {
    assert_unordered_eq(
        Solution::subdomain_visits(vec!["9001 discuss.leetcode.com".to_string()]),
        vec![
            "9001 leetcode.com".to_string(),
            "9001 discuss.leetcode.com".to_string(),
            "9001 com".to_string(),
        ],
    );

    assert_unordered_eq(
        Solution::subdomain_visits(vec![
            "900 google.mail.com".to_string(),
            "50 yahoo.com".to_string(),
            "1 intel.mail.com".to_string(),
            "5 wiki.org".to_string(),
        ]),
        vec![
            "901 mail.com".to_string(),
            "50 yahoo.com".to_string(),
            "900 google.mail.com".to_string(),
            "5 wiki.org".to_string(),
            "5 org".to_string(),
            "1 intel.mail.com".to_string(),
            "951 com".to_string(),
        ],
    );
}

fn assert_unordered_eq(mut actual: Vec<String>, mut expected: Vec<String>) {
    actual.sort();
    expected.sort();
    assert_eq!(actual, expected);
}

struct Solution{}
impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut counts: HashMap<String, i32> = HashMap::new();

        for cpdomain in cpdomains {
            let Some((count_str, domain)) = cpdomain.split_once(' ') else {
                continue;
            };

            let Ok(count) = count_str.parse::<i32>() else {
                continue;
            };

            let parts: Vec<&str> = domain.split('.').collect();
            for i in 0..parts.len() {
                let subdomain = parts[i..].join(".");
                *counts.entry(subdomain).or_insert(0) += count;
            }
        }

        counts
            .into_iter()
            .map(|(domain, count)| format!("{} {}", count, domain))
            .collect()
    }
}
