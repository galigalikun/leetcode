fn main() {
    assert_eq!(Solution::find_shortest_sub_array(vec![1,2,2,3,1]), 2);
    assert_eq!(Solution::find_shortest_sub_array(vec![1,2,2,3,1,4,2]), 6);
}

struct Solution{}
use std::collections::HashMap;
impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        let mut left = HashMap::new();
        let mut right = HashMap::new();
        let mut max_count = 0;
        let mut result = 0;
        for num in nums.clone() {
            *count.entry(num).or_insert(0) += 1;
            if *count.get(&num).unwrap() > max_count {
                max_count = *count.get(&num).unwrap();
            }
            *left.entry(num).or_insert(0) += 1;
            *right.entry(num).or_insert(0) += 1;
        }
        let mut min_length = nums.len() as i32;
        for (key, value) in count.iter() {
            if *value == max_count {
                let left_count = *left.get(key).unwrap();
                let right_count = *right.get(key).unwrap();
                let length = right_count - left_count + 1;
                if length < min_length {
                    min_length = length;
                    result = max_count;
                }
            }
        }

        result
    }
}
