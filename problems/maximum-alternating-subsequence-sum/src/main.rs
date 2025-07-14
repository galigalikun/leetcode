fn main() {
    assert_eq!(Solution::max_alternating_sum(vec![4, 2, 5, 3]), 7);
    assert_eq!(Solution::max_alternating_sum(vec![5, 6, 7, 8]), 15);
    assert_eq!(Solution::max_alternating_sum(vec![6, 2, 1, 2, 4, 5]), 10);
}

struct Solution;
impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        return nums
            .iter()
            .fold((0, 0), |(even, odd), &num| {
                (odd + num as i64, even.max(odd))
            })
            .1;
    }
}
