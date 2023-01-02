fn main() {
    assert_eq!(
        Solution::has_groups_size_x(vec![1, 2, 3, 4, 4, 3, 2, 1]),
        true
    );
    assert_eq!(
        Solution::has_groups_size_x(vec![1, 1, 1, 2, 2, 2, 3, 3]),
        false
    );
    assert_eq!(Solution::has_groups_size_x(vec![1]), false);
    assert_eq!(Solution::has_groups_size_x(vec![1, 1, 2, 2, 2, 2]), true);
    assert_eq!(
        Solution::has_groups_size_x(vec![1, 1, 1, 1, 2, 2, 2, 2, 2, 2]),
        true
    );
    assert_eq!(
        Solution::has_groups_size_x(vec![0, 0, 0, 1, 1, 1, 1, 1, 1, 2, 2, 2, 3, 3, 3]),
        true
    );
}

struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for d in deck {
            *map.entry(d).or_insert(0) += 1;
        }
        let mut p = 0;
        for m in map {
            if (p > 0 && m.1 % p != 0) || m.1 == 1 {
                return false;
            }
            p = if m.1 % 2 == 0 {
                2
            } else if m.1 % 3 == 0 {
                3
            } else {
                m.1
            };
        }

        return true;
    }
}
