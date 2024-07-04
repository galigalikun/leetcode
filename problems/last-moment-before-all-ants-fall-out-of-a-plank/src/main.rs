fn main() {
    assert_eq!(Solution::get_last_moment(4, vec![4, 3], vec![0, 1]), 4);
    assert_eq!(
        Solution::get_last_moment(7, vec![], vec![0, 1, 2, 3, 4, 5, 6, 7]),
        7
    );
    assert_eq!(
        Solution::get_last_moment(7, vec![0, 1, 2, 3, 4, 5, 6, 7], vec![]),
        7
    );
}

struct Solution;
impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        return n - left.iter().max().unwrap().max(right.iter().min().unwrap());
    }
}
