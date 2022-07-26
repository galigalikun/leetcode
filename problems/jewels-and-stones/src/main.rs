fn main() {
    assert_eq!(
        Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()),
        3
    );
    assert_eq!(
        Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string()),
        0
    );
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut map = HashMap::new();
        for c in jewels.chars() {
            map.insert(c, 1);
        }
        let mut ans = 0;
        for stone in stones.chars() {
            if map.get(&stone) != None {
                ans += 1;
            }
        }
        return ans;
    }
}
