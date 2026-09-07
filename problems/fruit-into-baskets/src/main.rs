use std::collections::HashMap;

fn main() {
    assert_eq!(Solution::total_fruit(vec![1,2,1]), 3);
    assert_eq!(Solution::total_fruit(vec![0,1,2,2]), 3);
    assert_eq!(Solution::total_fruit(vec![1,2,3,2,2]), 4);
}

struct Solution;
impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut counts = HashMap::new();
        let mut left = 0usize;
        let mut max_len = 0usize;

        for right in 0..fruits.len() {
            let fruit = fruits[right];
            *counts.entry(fruit).or_insert(0usize) += 1;

            while counts.len() > 2 {
                let left_fruit = fruits[left];
                if let Some(count) = counts.get_mut(&left_fruit) {
                    *count -= 1;
                    if *count == 0 {
                        counts.remove(&left_fruit);
                    }
                }
                left += 1;
            }

            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }
}
