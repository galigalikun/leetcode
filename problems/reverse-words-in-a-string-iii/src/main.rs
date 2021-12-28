fn main() {
    assert_eq!(
        Solution::reverse_words("Let's take LeetCode contest".to_string()),
        "s'teL ekat edoCteeL tsetnoc"
    );
    assert_eq!(Solution::reverse_words("God Ding".to_string()), "doG gniD");
}

struct Solution {}
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut ans = String::new();
        for w in s.split(" ") {
            ans = format!("{}{} ", ans, w.chars().rev().collect::<String>());
        }
        return ans.trim_end().to_string();
    }
}
