fn main() {
    assert_eq!(Solution::shortest_subarray(vec![1], 1), 1);
    assert_eq!(Solution::shortest_subarray(vec![1,2], 4), -1);
    assert_eq!(Solution::shortest_subarray(vec![2,-1,2], 3), 3);
}

struct Solution;
impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        return -1;
    }
}
