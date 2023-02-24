fn main() {
    assert_eq!(Solution::can_reorder_doubled(vec![3,1,3,6]), false);
    assert_eq!(Solution::can_reorder_doubled(vec![2,1,2,6]), false);
    assert_eq!(Solution::can_reorder_doubled(vec![4,-2,2,-4]), true);
}

struct Solution;
impl Solution {
    pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
        return false;
    }
}
