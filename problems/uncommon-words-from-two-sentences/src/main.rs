fn main() {
    assert_eq!(Solution::uncommon_from_sentences("this apple is sweet".to_string(), "this apple is sour".to_string()), ["sweet","sour"]);
    assert_eq!(Solution::uncommon_from_sentences("apple apple".to_string(), "banana".to_string()), vec!["banana"]);
}

struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut map = HashMap::new();
        for s in s1.split(" ") {
            *map.entry(s).or_insert(0) += 1;
        }
        for s in s2.split(" ") {
            *map.entry(s).or_insert(0) += 1;
        }
        let mut ans = vec![];
        for (a, b) in map {
            if b == 1 {
                ans.push(a.to_string());
            }
        }
        return ans;
    }
}
