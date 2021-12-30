fn main() {
    // assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
    // assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
    assert_eq!(Solution::subarray_sum(vec![1], 0), 0);
    assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
}

// https://qiita.com/KueharX/items/ed07eb4d7a967baa6d1c
struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();
        map.insert(0, 1);
        let mut ans = 0;
        let mut calc = 0;
        for n in nums {
            calc += n;
            if let Some(m) = map.get_mut(&calc) {
                *m += 1;
            } else {
                map.insert(calc, 1);
            }
            ans += map.get(&(calc - k)).unwrap_or_else(|| &0);
        }
        println!("debug {:?}", map);
        return ans;
    }
}
