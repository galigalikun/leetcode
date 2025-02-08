fn main() {
    assert_eq!(Solution::count_balls(1, 10), 2);
    assert_eq!(Solution::count_balls(5, 15), 2);
    assert_eq!(Solution::count_balls(19, 28), 2);
}

struct Solution;
impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for i in low_limit..=high_limit {
            let mut sum = 0;
            let mut i = i;
            while i > 0 {
                sum += i % 10;
                i /= 10;
            }
            *map.entry(sum).or_insert(0) += 1;
        }
        *map.values().max().unwrap()
    }
}
