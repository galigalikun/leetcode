fn main() {
    assert_eq!(
        Solution::find_rotate_steps("godding".to_string(), "gd".to_string()),
        4
    );
    assert_eq!(
        Solution::find_rotate_steps("godding".to_string(), "godding".to_string()),
        13
    );
}

struct Solution {}
// https://cheonhyangzhang.gitbooks.io/leetcode-solutions/content/solutions-501-550/514-freedom-trail.html
use std::collections::HashMap;
impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let mut map: HashMap<char, Vec<i32>> = HashMap::new();
        let mut i = 0;
        for c in ring.chars() {
            if let Some(m) = map.get_mut(&c) {
                (*m).push(i);
            } else {
                map.insert(c, vec![i]);
            }
            i += 1;
        }
        let mut poss = vec![0];
        let mut ways = vec![0];
        let mut ans = 0;
        for c in key.chars() {
            if let Some(m) = map.get(&c) {
                let mut newposs = vec![];
                let mut newways = vec![];
                ans = std::i32::MAX;
                for &pos in m {
                    let mut min = std::i32::MAX;
                    for j in 0..poss.len() {
                        min = std::cmp::min(
                            min,
                            std::cmp::min(
                                std::cmp::max(pos, poss[j]) - std::cmp::min(pos, poss[j]),
                                ring.len() as i32 - std::cmp::max(pos, poss[j])
                                    + std::cmp::min(pos, poss[j]),
                            ) + ways[j],
                        );
                    }
                    min += 1;
                    newposs.push(pos);
                    newways.push(min);
                    ans = std::cmp::min(ans, min);
                }
                poss = newposs;
                ways = newways;
            }
        }

        return ans;
    }
}
