fn main() {
    assert_eq!(
        Solution::reachable_nodes(vec![vec![0, 1, 10], vec![0, 2, 1], vec![1, 2, 2]], 6, 3),
        13
    );
    assert_eq!(
        Solution::reachable_nodes(
            vec![vec![0, 1, 4], vec![1, 2, 6], vec![0, 2, 8], vec![1, 3, 1]],
            10,
            4
        ),
        23
    );
    assert_eq!(
        Solution::reachable_nodes(
            vec![
                vec![1, 2, 4],
                vec![1, 4, 5],
                vec![1, 3, 1],
                vec![2, 3, 4],
                vec![3, 4, 5]
            ],
            17,
            5
        ),
        1
    );
}

struct Solution;
impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        return 0;
    }
}
