fn main() {
    assert_eq!(
        Solution::evaluate(
            "(name)is(age)yearsold".to_string(),
            vec![
                vec!["name".to_string(), "bob".to_string()],
                vec!["age".to_string(), "two".to_string()]
            ]
        ),
        "bobistwoyearsold"
    );
    assert_eq!(
        Solution::evaluate(
            "hi(name)".to_string(),
            vec![vec!["name".to_string(), "bob".to_string()]]
        ),
        "hibob"
    );
    assert_eq!(
        Solution::evaluate(
            "(a)(a)(a)aaa".to_string(),
            vec![vec!["a".to_string(), "yes".to_string()]]
        ),
        "yesyesyesaaa"
    );
}

struct Solution;
impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let mut map = std::collections::HashMap::new();
        for k in knowledge {
            map.insert(k[0].clone(), k[1].clone());
        }
        let mut result = String::new();
        let mut i = 0;
        while i < s.len() {
            if s.chars().nth(i) == Some('(') {
                let mut j = i + 1;
                while j < s.len() && s.chars().nth(j) != Some(')') {
                    j += 1;
                }
                let key = &s[i + 1..j];
                if let Some(value) = map.get(key) {
                    result.push_str(value);
                } else {
                    result.push_str("?");
                }
                i = j + 1;
            } else {
                result.push(s.chars().nth(i).unwrap());
                i += 1;
            }
        }
        result
    }
}
