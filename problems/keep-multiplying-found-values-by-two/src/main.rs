fn main() {
    assert_eq!(Solution::find_final_value(vec![5, 3, 6, 1, 12], 3), 24);
    assert_eq!(Solution::find_final_value(vec![2, 7, 9], 4), 4);
}

struct Solution;
impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut value = original;
        let num_set: std::collections::HashSet<i32> = nums.into_iter().collect();

        while num_set.contains(&value) {
            value *= 2;
        }

        value
    }
}
