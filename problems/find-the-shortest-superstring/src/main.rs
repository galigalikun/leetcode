fn main() {
    assert_eq!(Solution::shortest_superstring(vec!["alex".to_string(),"loves".to_string(),"leetcode".to_string()]), "alexlovesleetcode");
    assert_eq!(Solution::shortest_superstring(vec!["catg".to_string(),"ctaagt".to_string(),"gcta".to_string(),"ttca".to_string(),"atgcatc".to_string()]), "gctaagttcatgcatc");
}

struct Solution;
impl Solution {
    pub fn shortest_superstring(words: Vec<String>) -> String {
        return "".to_string();
    }
}
