fn main() {
    assert_eq!(Solution::k_closest(vec![vec![1,3],vec![-2,2]], 1), vec![[-2,2]]);
    assert_eq!(Solution::k_closest(vec![vec![3,3],vec![5,-1],vec![-2,4]], 2), vec![[3,3],[-2,4]]);
}

struct Solution;
impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        return vec![];
    }
}
