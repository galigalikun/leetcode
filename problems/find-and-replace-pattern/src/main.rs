fn main() {
    assert_eq!(Solution::find_and_replace_pattern(vec!["abc".to_string(),"deq".to_string(),"mee".to_string(),"aqq".to_string(),"dkd".to_string(),"ccc".to_string()], "abb".to_string()), vec!["mee","aqq"]);
    assert_eq!(Solution::find_and_replace_pattern(vec!["a".to_string(),"b".to_string(),"c".to_string()], "a".to_string()), vec!["a","b","c"]);
}

struct Solution;
impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        return vec![];
    }
}
