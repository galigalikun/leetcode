fn main() {
    assert_eq!(Solution::can_construct("annabelle".to_string(), 2), true);
    assert_eq!(Solution::can_construct("leetcode".to_string(), 3), false);
    assert_eq!(Solution::can_construct("true".to_string(), 4), true);
    assert_eq!(Solution::can_construct("cr".to_string(), 7), false);
}

struct Solution;
impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let mut map = std::collections::HashMap::new();
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        let mut odd = 0;
        for (_, v) in map {
            if v % 2 == 1 {
                odd += 1;
            }
        }
        if odd <= k {
            return true;
        }
        return false;
    }
}
