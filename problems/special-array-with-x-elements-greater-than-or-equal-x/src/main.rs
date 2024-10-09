fn main() {
    assert_eq!(Solution::special_array(vec![3, 5]), 2);
    assert_eq!(Solution::special_array(vec![0, 0]), -1);
    assert_eq!(Solution::special_array(vec![0, 4, 3, 0, 4]), 3);
    assert_eq!(Solution::special_array(vec![3, 6, 7, 7, 0]), -1);
    assert_eq!(Solution::special_array(vec![3]), 1);
}

struct Solution;
impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let mut end = 0;
        for &n in &nums {
            *map.entry(n).or_insert(0) += 1;
            end = end.max(n);
        }
        for x in 0..=end {
            let total = map
                .iter()
                .filter(|(&k, _)| k >= x)
                .map(|(_, &v)| v)
                .sum::<i32>();
            if x == total {
                return x;
            }
        }
        -1
    }
}
