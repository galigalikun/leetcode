fn main() {
    assert_eq!(Solution::last_substring("abab".to_string()), "bab");
    assert_eq!(Solution::last_substring("leetcode".to_string()), "tcode");
    assert_eq!(Solution::last_substring("cacacb".to_string()), "cb");
}

struct Solution;
impl Solution {
    pub fn last_substring(s: String) -> String {
        let mut s = s.chars().collect::<Vec<char>>();
        let mut max = s[0];
        let mut max_index = 0;
        for i in 1..s.len() {
            if s[i] > max {
                max = s[i];
                max_index = i;
            }
        }
        s = s[max_index..].to_vec();
        return s.iter().collect::<String>();
    }
}
