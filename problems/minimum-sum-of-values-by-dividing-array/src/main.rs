fn main() {
    assert_eq!(
        Solution::minimum_value_sum(vec![1, 4, 3, 3, 2], vec![0, 3, 3, 2]),
        12
    );
    assert_eq!(
        Solution::minimum_value_sum(vec![2, 3, 5, 7, 7, 7, 5], vec![0, 7, 5]),
        17
    );
    assert_eq!(Solution::minimum_value_sum(vec![1, 2, 3, 4], vec![2]), -1);
}

struct Solution;
impl Solution {
    pub fn minimum_value_sum(nums: Vec<i32>, and_values: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut result = 0;

        for bit in 0..30 {
            let bit_mask = 1 << bit;
            let mut count_and_set = 0;
            let mut count_nums_set = 0;

            for &val in &and_values {
                if val & bit_mask != 0 {
                    count_and_set += 1;
                }
            }

            for &num in &nums {
                if num & bit_mask != 0 {
                    count_nums_set += 1;
                }
            }

            if count_and_set > 0 {
                result += bit_mask;
            } else if count_nums_set < n {
                // We can set this bit to 0 in all nums
                continue;
            } else {
                // All nums have this bit set, but and_values do not require it
                // So we can set this bit to 0 in all nums
                continue;
            }
        }

        result
    }
}
