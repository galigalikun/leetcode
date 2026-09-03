fn main() {
    assert_eq!(Solution::subarray_bitwise_o_rs(vec![0]), 1);
    assert_eq!(Solution::subarray_bitwise_o_rs(vec![1,1,2]), 3);
    assert_eq!(Solution::subarray_bitwise_o_rs(vec![1, 2, 4]), 6);
    assert_eq!(Solution::subarray_bitwise_o_rs(vec![3]), 1);
}

struct Solution;
impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let mut all_values: HashSet<i32> = HashSet::new();
        let mut prev_end_values: HashSet<i32> = HashSet::new();

        for value in arr {
            let mut curr_end_values: HashSet<i32> = HashSet::new();
            curr_end_values.insert(value);

            for prev in &prev_end_values {
                curr_end_values.insert(prev | value);
            }

            for current in &curr_end_values {
                all_values.insert(*current);
            }

            prev_end_values = curr_end_values;
        }

        all_values.len() as i32
    }
}
