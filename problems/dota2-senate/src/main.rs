fn main() {
    assert_eq!(Solution::predict_party_victory("RD".to_string()), "Radiant");
    assert_eq!(Solution::predict_party_victory("RDD".to_string()), "Dire");
    assert_eq!(Solution::predict_party_victory("DR".to_string()), "Dire");
    assert_eq!(Solution::predict_party_victory("DDRRR".to_string()), "Dire");
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut map = HashMap::new();
        for s in senate.chars() {
            if let Some(m) = map.get_mut(&s) {
                *m += 1;
            } else {
                map.insert(s, 1);
            }
        }
        if map.get(&'R') == map.get(&'D') {
            if &senate[0..1] == "R" {
                return "Radiant".to_string();
            }
            return "Dire".to_string();
        } else if map.get(&'R') > map.get(&'D') {
            return "Radiant".to_string();
        }
        return "Dire".to_string();
    }
}
