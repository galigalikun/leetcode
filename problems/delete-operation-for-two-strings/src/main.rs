fn main() {
    assert_eq!(
        Solution::min_distance("sea".to_string(), "eat".to_string()),
        2
    );
    assert_eq!(
        Solution::min_distance("leetcode".to_string(), "etco".to_string()),
        4
    );
    assert_eq!(Solution::min_distance("a".to_string(), "a".to_string()), 0);
}

struct Solution {}
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut result = word1.chars().collect::<Vec<_>>();
        let mut ans = 0;
        for w in word2.chars() {
            if let Some(p) = result.iter().position(|&c| c == w) {
                result.remove(p);
                ans += 1;
            }
        }
        return ans;
    }
}
