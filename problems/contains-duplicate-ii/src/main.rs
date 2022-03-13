fn main() {
    // assert_eq!(
    //     Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3),
    //     true
    // );
    // assert_eq!(
    //     Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1),
    //     true
    // );
    // assert_eq!(
    //     Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2),
    //     false
    // );
    assert_eq!(Solution::contains_nearby_duplicate(vec![99, 99], 2), true);
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        for i in 0..nums.len() {
            if let Some(m) = map.get_mut(&nums[i]) {
                (*m).push(i as i32);
            } else {
                map.insert(nums[i], vec![i as i32]);
            }
        }
        for (_, list) in map {
            for i in 0..list.len() {
                for j in 0..list.len() {
                    if i != j {
                        if (list[i] - list[j]).abs() <= k {
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }
}
