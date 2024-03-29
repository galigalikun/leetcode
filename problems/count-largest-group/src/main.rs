fn main() {
    assert_eq!(Solution::count_largest_group(13), 4);
    assert_eq!(Solution::count_largest_group(2), 2);
}

struct Solution;
impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let mut max = 0;
        for i in 1..=n {
            let mut sum = 0;
            let mut i = i;
            while i > 0 {
                sum += i % 10;
                i /= 10;
            }
            let count = map.entry(sum).or_insert(0);
            *count += 1;
            max = max.max(*count);
        }
        let mut res = 0;
        for &v in map.values() {
            if v == max {
                res += 1;
            }
        }
        return res;
    }
}
