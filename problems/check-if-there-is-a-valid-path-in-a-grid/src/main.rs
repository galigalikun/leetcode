fn main() {
    assert_eq!(Solution::has_valid_path(vec![vec![2,4,3],vec![6,5,2]]), true);
    assert_eq!(Solution::has_valid_path(vec![vec![1,2,1],vec![1,2,1]]), false);
    assert_eq!(Solution::has_valid_path(vec![vec![1,1,2]]), false);
}

struct Solution;
impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        return false;
    }
}
