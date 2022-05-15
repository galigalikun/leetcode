fn main() {
    assert_eq!(Solution::find_shortest_sub_array(vec![1,2,2,3,1]), 2);
    assert_eq!(Solution::find_shortest_sub_array(vec![1,2,2,3,1,4,2]), 6);
}

struct Solution{}
use std::collections::HashMap;
impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        let mut max_count = 0;
        for num in nums.clone() {
            *count.entry(num).or_insert(0) += 1;
            if *count.get(&num).unwrap() > max_count {
                max_count = *count.get(&num).unwrap();
            }
        }
        println!("debug {:?} {}", count, max_count);
        let mut min_length = nums.len() as i32;
        for num in count.keys() {
            if *count.get(num).unwrap() == max_count {
                min_length = std::cmp::min(min_length, *num+1);
            }
        }

        min_length
    }
}
