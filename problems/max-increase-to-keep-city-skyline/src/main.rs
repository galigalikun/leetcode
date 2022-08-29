fn main() {
    assert_eq!(Solution::max_increase_keeping_skyline(vec![vec![3,0,8,4],vec![2,4,5,7],vec![9,2,6,3],vec![0,3,1,0]]), 35);
    assert_eq!(Solution::max_increase_keeping_skyline(vec![vec![0,0,0],vec![0,0,0],vec![0,0,0]]), 0);
}

struct Solution{}
impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        return -1;
    }
}
