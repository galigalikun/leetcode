fn main() {
    assert_eq!(
        Solution::count_matches(
            vec![
                vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
                vec![
                    "computer".to_string(),
                    "silver".to_string(),
                    "lenovo".to_string()
                ],
                vec![
                    "phone".to_string(),
                    "gold".to_string(),
                    "iphone".to_string()
                ]
            ],
            "color".to_string(),
            "silver".to_string()
        ),
        1
    );
    assert_eq!(
        Solution::count_matches(
            vec![
                vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
                vec![
                    "computer".to_string(),
                    "silver".to_string(),
                    "phone".to_string()
                ],
                vec![
                    "phone".to_string(),
                    "gold".to_string(),
                    "iphone".to_string()
                ]
            ],
            "type".to_string(),
            "phone".to_string()
        ),
        2
    );
}

struct Solution;
impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        return items
            .iter()
            .filter(|item| match rule_key.as_str() {
                "type" => item[0] == rule_value,
                "color" => item[1] == rule_value,
                "name" => item[2] == rule_value,
                _ => false,
            })
            .count() as i32;
    }
}
