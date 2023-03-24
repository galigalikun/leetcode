fn main() {
    assert_eq!(
        Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]]),
        2
    );
    assert_eq!(
        Solution::unique_paths_iii(vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]]),
        4
    );
    assert_eq!(Solution::unique_paths_iii(vec![vec![0, 1], vec![2, 0]]), 0);
}

struct Solution;
impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        return 0;
    }
}
