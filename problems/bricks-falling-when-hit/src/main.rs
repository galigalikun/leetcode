fn main() {
    assert_eq!(Solution::hit_bricks(vec![vec![1,0,0,0],vec![1,1,1,0]], vec![vec![1,0]]), vec![2]);
    assert_eq!(Solution::hit_bricks(vec![vec![1,0,0,0],vec![1,1,0,0]], vec![vec![1,1],vec![1,0]]), vec![0,0]);
}

struct Solution{}
impl Solution {
    pub fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        return vec![];
    }
}
