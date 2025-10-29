fn main() {
    assert_eq!(Solution::min_operations(vec![vec![2, 4], vec![6, 8]], 2), 4);
    assert_eq!(Solution::min_operations(vec![vec![1, 5], vec![2, 3]], 1), 5);
    assert_eq!(
        Solution::min_operations(vec![vec![1, 2], vec![3, 4]], 2),
        -1
    );
}

struct Solution;
impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let first = grid[0][0];
        let mut diffs = vec![];
        for row in &grid {
            for &val in row {
                let diff = val - first;
                if diff % x != 0 {
                    return -1;
                }
                diffs.push(diff / x);
            }
        }
        diffs.sort_unstable();
        let median = diffs[diffs.len() / 2];
        let mut operations = 0;
        for diff in diffs {
            operations += (diff - median).abs();
        }
        operations
    }
}
