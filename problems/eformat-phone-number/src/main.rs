fn main() {
    assert_eq!(
        Solution::reformat_number("1-23-45 6".to_string()),
        "123-456".to_string()
    );
    assert_eq!(
        Solution::reformat_number("123 4-567".to_string()),
        "123-45-67".to_string()
    );
    assert_eq!(
        Solution::reformat_number("123 4-5678.9".to_string()),
        "123-456-789".to_string()
    );
}

struct Solution;
impl Solution {
    pub fn reformat_number(number: String) -> String {
        let binding = number
            .chars()
            .filter(|&c| c != ' ' && c != '-' && c != '.')
            .collect::<String>()
            .chars()
            .collect::<Vec<char>>();

        let mut result = String::new();
        let mut i = 0;

        while i < binding.len() {
            let remaining = binding.len() - i;
            if remaining > 4 {
                result.push_str(&binding[i..i + 3].iter().collect::<String>());
                result.push('-');
                i += 3;
            } else if remaining == 4 {
                result.push_str(&binding[i..i + 2].iter().collect::<String>());
                result.push('-');
                result.push_str(&binding[i + 2..i + 4].iter().collect::<String>());
                break;
            } else {
                result.push_str(&binding[i..].iter().collect::<String>());
                break;
            }
        }

        result
    }
}
