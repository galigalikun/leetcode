fn main() {
    assert_eq!(
        Solution::truncate_sentence("Hello how are you Contestant".to_string(), 4),
        "Hello how are you"
    );
    assert_eq!(
        Solution::truncate_sentence("What is the solution to this problem".to_string(), 4),
        "What is the solution"
    );
}

struct Solution;
impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        return s
            .split_whitespace()
            .take(k as usize)
            .collect::<Vec<&str>>()
            .join(" ");
    }
}
