fn main() {
    assert_eq!(Solution::min_area_free_rect(vec![vec![1,2],vec![2,1],vec![1,0],vec![0,1]]), 2.0);
    assert_eq!(Solution::min_area_free_rect(vec![vec![0,1],vec![2,1],vec![1,1],vec![1,0],vec![2,0]]), 1.0);
    assert_eq!(Solution::min_area_free_rect(vec![vec![0,3],vec![1,2],vec![3,1],vec![1,3],vec![2,1]]), 0.0);
}

struct Solution;
impl Solution {
    pub fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64 {
        return 0.0;
    }
}
