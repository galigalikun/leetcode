fn main() {
    assert_eq!(
        Solution::entity_parser("&amp; is an HTML entity but &ambassador; is not.".to_string()),
        "& is an HTML entity but &ambassador; is not."
    );
    assert_eq!(
        Solution::entity_parser("and I quote: &quot;...&quot;".to_string()),
        "and I quote: \"...\""
    )
}

struct Solution;
impl Solution {
    pub fn entity_parser(text: String) -> String {
        let mut result = String::new();
        let mut i = 0;
        let n = text.len();
        while i < n {
            if text[i..].starts_with("&quot;") {
                result.push('"');
                i += 6;
            } else if text[i..].starts_with("&apos;") {
                result.push('\'');
                i += 6;
            } else if text[i..].starts_with("&amp;") {
                result.push('&');
                i += 5;
            } else if text[i..].starts_with("&gt;") {
                result.push('>');
                i += 4;
            } else if text[i..].starts_with("&lt;") {
                result.push('<');
                i += 4;
            } else if text[i..].starts_with("&frasl;") {
                result.push('/');
                i += 7;
            } else {
                result.push(text.chars().nth(i).unwrap());
                i += 1;
            }
        }
        return result;
    }
}
