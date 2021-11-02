fn main() {
    assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
    assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
    assert_eq!(
        Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
        true
    );
}

pub struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map: HashMap<i32, bool> = HashMap::new();
        for n in nums {
            if None == map.get(&n) {
                map.insert(n, true);
            } else {
                return true;
            }
        }
        return false;
    }
}
