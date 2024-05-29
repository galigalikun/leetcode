fn main() {
    assert_eq!(
        Solution::can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3]),
        true
    );
    assert_eq!(Solution::can_be_equal(vec![7], vec![7]), true);
    assert_eq!(Solution::can_be_equal(vec![3, 7, 9], vec![3, 7, 11]), false);
    assert_eq!(Solution::can_be_equal(vec![1, 2, 3], vec![2, 2, 2]), false);
}

struct Solution;
impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        if target.len() != arr.len() {
            return false;
        }
        let mut target = target;
        let mut arr = arr;
        target.sort();
        arr.sort();
        return target == arr;
    }
}
