fn main() {
    assert_eq!(Solution::shortest_bridge(vec![vec![0, 1], vec![1, 0]]), 1);
    assert_eq!(
        Solution::shortest_bridge(vec![vec![0, 1, 0], vec![0, 0, 0], vec![0, 0, 1]]),
        2
    );
    assert_eq!(
        Solution::shortest_bridge(vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1]
        ]),
        1
    );
}

struct Solution;
impl Solution {
    pub fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
        return 0;
    }
}
