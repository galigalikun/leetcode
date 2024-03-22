fn main() {
    assert_eq!(Solution::find_lucky(vec![2, 2, 3, 4]), 2);
    assert_eq!(Solution::find_lucky(vec![1, 2, 2, 3, 3, 3]), 3);
    assert_eq!(Solution::find_lucky(vec![2, 2, 2, 3, 3]), -1);
    assert_eq!(Solution::find_lucky(vec![1, 2, 2, 3, 3, 3]), 3);
}

struct Solution;
impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for &num in &arr {
            *map.entry(num).or_insert(0) += 1;
        }
        let mut max = None;
        for (&num, &count) in &map {
            if num == count {
                if max.is_none() || max.unwrap() < num {
                    max = Some(num);
                }
            }
        }
        if let Some(max) = max {
            return max;
        }
        return -1;
    }
}
