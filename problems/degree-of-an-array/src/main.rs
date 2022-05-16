fn main() {
    assert_eq!(Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1]), 2);
    assert_eq!(
        Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1, 4, 2]),
        6
    );
}

// https://chih-hsien.gitbooks.io/mysolution/content/degree-of-an-array.html
struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        let mut left = HashMap::new();
        let mut right = HashMap::new();
        let mut max_count = 0;
        for (i, num) in nums.iter().enumerate() {
            if !count.contains_key(&num) {
                *left.entry(*num).or_insert(i) = i;
            }
            *count.entry(num).or_insert(0) += 1;
            *right.entry(*num).or_insert(i) = i;
            max_count = std::cmp::max(max_count, *count.get(&num).unwrap());
        }
        let mut result = nums.len() as i32;
        for (key, value) in count.iter() {
            if *value == max_count {
                result = std::cmp::min(
                    result,
                    (right.get(key).unwrap() - left.get(key).unwrap() + 1) as i32,
                );
            }
        }

        return result;
    }
}
