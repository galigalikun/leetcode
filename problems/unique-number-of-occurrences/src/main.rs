fn main() {
    assert_eq!(Solution::unique_occurrences(vec![1,2,2,1,1,3]), true);
    assert_eq!(Solution::unique_occurrences(vec![1,2]), false);
    assert_eq!(Solution::unique_occurrences(vec![-3,0,1,-3,1,1,1,-3,10,0]), true);
}

struct Solution;
use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {

        let mut map = HashMap::new();
        for i in arr {
            let count = map.entry(i).or_insert(0);
            *count += 1;
        }
        let mut set = HashSet::new();
        for (_, v) in map {
            if set.contains(&v) {
                return false;
            }
            set.insert(v);
        }
        return true;
    }
}
