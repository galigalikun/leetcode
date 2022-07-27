fn main() {
    assert_eq!(Solution::sliding_puzzle(vec![vec![1,2,3],vec![4,0,5]]), 1);
    assert_eq!(Solution::sliding_puzzle(vec![vec![1,2,3],vec![5,4,0]]), -1);
    assert_eq!(Solution::sliding_puzzle(vec![vec![4,1,2],vec![5,0,3]]), 5);
}

struct Solution{}
impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        return -1;
    }
}
