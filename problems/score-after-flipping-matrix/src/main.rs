fn main() {
    assert_eq!(Solution::matrix_score(vec![vec![0,0,1,1],vec![1,0,1,0],vec![1,1,0,0]]), 39);
    assert_eq!(Solution::matrix_score(vec![vec![0]]), 1);
}

struct Solution;
impl Solution {
    pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
        return 0;
    }
}
