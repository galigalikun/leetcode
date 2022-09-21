fn main() {
    assert_eq!(Solution::largest_island(vec![vec![1,0],vec![0,1]]), 3);
    assert_eq!(Solution::largest_island(vec![vec![1,1],vec![1,0]]), 4);
    assert_eq!(Solution::largest_island(vec![vec![1,1],vec![1,1]]), 4);
}

struct Solution{}
impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        return 0;
    }
}
