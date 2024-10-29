fn main() {
    assert_eq!(
        Solution::minimum_effort_path(vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]]),
        2
    );
    assert_eq!(
        Solution::minimum_effort_path(vec![vec![1, 2, 3], vec![3, 8, 4], vec![5, 3, 5]]),
        1
    );
    assert_eq!(
        Solution::minimum_effort_path(vec![
            vec![1, 2, 1, 1, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 1, 1, 2, 1]
        ]),
        0
    );
}

struct Solution;
impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let mut heap = std::collections::BinaryHeap::new();
        let mut visited = vec![vec![false; heights[0].len()]; heights.len()];
        let mut effort = vec![vec![std::i32::MAX; heights[0].len()]; heights.len()];
        heap.push((0, 0, 0));
        effort[0][0] = 0;
        while let Some((e, x, y)) = heap.pop() {
            if visited[x][y] {
                continue;
            }
            visited[x][y] = true;
            if x > 0 {
                let diff = (heights[x][y] - heights[x - 1][y]).abs();
                let max_diff = std::cmp::max(diff, effort[x][y]);
                if max_diff < effort[x - 1][y] {
                    effort[x - 1][y] = max_diff;
                    heap.push((max_diff, x - 1, y));
                }
            }
            if x < heights.len() - 1 {
                let diff = (heights[x][y] - heights[x + 1][y]).abs();
                let max_diff = std::cmp::max(diff, effort[x][y]);
                if max_diff < effort[x + 1][y] {
                    effort[x + 1][y] = max_diff;
                    heap.push((max_diff, x + 1, y));
                }
            }
            if y > 0 {
                let diff = (heights[x][y] - heights[x][y - 1]).abs();
                let max_diff = std::cmp::max(diff, effort[x][y]);
                if max_diff < effort[x][y - 1] {
                    effort[x][y - 1] = max_diff;
                    heap.push((max_diff, x, y - 1));
                }
            }
            if y < heights[0].len() - 1 {
                let diff = (heights[x][y] - heights[x][y + 1]).abs();
                let max_diff = std::cmp::max(diff, effort[x][y]);
                if max_diff < effort[x][y + 1] {
                    effort[x][y + 1] = max_diff;
                    heap.push((max_diff, x, y + 1));
                }
            }
        }
        return effort[heights.len() - 1][heights[0].len() - 1];
    }
}
