fn main() {
    assert_eq!(Solution::remove_stones(vec![vec![0,0],vec![0,1],vec![1,0],vec![1,2],vec![2,1],vec![2,2]]), 5);
    assert_eq!(Solution::remove_stones(vec![vec![0,0],vec![0,2],vec![1,1],vec![2,0],vec![2,2]]), 3);
    assert_eq!(Solution::remove_stones(vec![vec![0,0]]), 0);
}

struct Solution;
impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        return 0;
    }
}
