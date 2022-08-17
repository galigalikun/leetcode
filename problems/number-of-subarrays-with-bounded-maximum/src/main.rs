fn main() {
    assert_eq!(Solution::num_subarray_bounded_max(vec![2,1,4,3], 2, 3), 3);
    assert_eq!(Solution::num_subarray_bounded_max(vec![2,9,2,5,6], 2, 8), 7);
}

struct Solution{}
impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        return 0;
    }
}
