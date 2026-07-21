fn main() {
    assert_eq!(Solution::num_subarray_bounded_max(vec![2,1,4,3], 2, 3), 3);
    assert_eq!(Solution::num_subarray_bounded_max(vec![2,9,2,5,6], 2, 8), 7);
}

struct Solution{}
impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        (Self::count_with_max_at_most(&nums, right)
            - Self::count_with_max_at_most(&nums, left - 1)) as i32
    }

    fn count_with_max_at_most(nums: &[i32], bound: i32) -> i64 {
        nums.iter()
            .fold((0_i64, 0_i64), |(total, current), &num| {
                let next_current = if num <= bound { current + 1 } else { 0 };
                (total + next_current, next_current)
            })
            .0
    }
}
