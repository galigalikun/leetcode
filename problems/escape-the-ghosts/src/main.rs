fn main() {
    assert_eq!(Solution::escape_ghosts(vec![[1,0],[0,3]], vec![0,1]), true);
    assert_eq!(Solution::escape_ghosts(vec![[1,0]], vec![2,0]), false);
    assert_eq!(Solution::escape_ghosts(vec![[2,0]], vec![1,0]), false);
}

struct Solution{}
impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        return false;
    }
}
