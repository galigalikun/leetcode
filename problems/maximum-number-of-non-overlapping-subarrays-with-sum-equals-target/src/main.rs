fn main() {
    assert_eq!(Solution::max_non_overlapping(vec![1, 1, 1, 1, 1], 2), 2);
    assert_eq!(
        Solution::max_non_overlapping(vec![-1, 3, 5, 1, 4, 2, -9], 6),
        2
    );
}

struct Solution;
impl Solution {
    pub fn max_non_overlapping(nums: Vec<i32>, target: i32) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(0, 0);
        let mut sum = 0;
        let mut last = 0;
        let mut count = 0;
        for num in nums {
            sum += num;
            if let Some(&i) = map.get(&(sum - target)) {
                last = last.max(i + 1);
            }
            if let Some(&i) = map.get(&sum) {
                map.insert(sum, i.max(last));
            } else {
                map.insert(sum, last);
            }
            if last > 0 {
                count += 1;
            }
        }
        return count;
    }
}
