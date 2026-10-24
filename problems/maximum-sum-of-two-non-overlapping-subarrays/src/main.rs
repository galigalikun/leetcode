fn main() {
    assert_eq!(
        Solution::max_sum_two_no_overlap(vec![0, 6, 5, 2, 2, 5, 1, 9, 4], 1, 2),
        20
    );
    assert_eq!(
        Solution::max_sum_two_no_overlap(vec![3, 8, 1, 3, 2, 1, 8, 9, 0], 3, 2),
        29
    );
    assert_eq!(
        Solution::max_sum_two_no_overlap(vec![2, 1, 5, 6, 0, 9, 5, 0, 3, 8], 4, 3),
        31
    );
    assert_eq!(
        Solution::max_sum_two_no_overlap(vec![5, 1, 1, 1, 10], 1, 3),
        17
    );
}

struct Solution;
impl Solution {
    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        let first_len = first_len as usize;
        let second_len = second_len as usize;

        let mut prefix = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            prefix[i + 1] = prefix[i] + nums[i];
        }

        let first_before_second = Self::max_ordered_sum(&prefix, first_len, second_len);
        let second_before_first = Self::max_ordered_sum(&prefix, second_len, first_len);
        first_before_second.max(second_before_first)
    }

    fn max_ordered_sum(prefix: &[i32], left_len: usize, right_len: usize) -> i32 {
        let mut best_left_sum = 0;
        let mut best_total = 0;

        for right_end in (left_len + right_len)..prefix.len() {
            let left_end = right_end - right_len;
            let left_sum = prefix[left_end] - prefix[left_end - left_len];
            best_left_sum = best_left_sum.max(left_sum);

            let right_sum = prefix[right_end] - prefix[right_end - right_len];
            best_total = best_total.max(best_left_sum + right_sum);
        }

        best_total
    }
}
