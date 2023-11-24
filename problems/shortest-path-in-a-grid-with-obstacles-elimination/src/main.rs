fn main() {
    assert_eq!(Solution::shortest_path(vec![vec![0,0,0],vec![1,1,0],vec![0,0,0],vec![0,1,1],vec![0,0,0]], 1), 6);
    assert_eq!(Solution::shortest_path(vec![vec![0,1,1],vec![1,1,1],vec![1,0,0]], 1), -1);
}

struct Solution;
impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, 0, 0, 0));
        while let Some((x, y, step, _count)) = queue.pop_front() {
            if x == grid.len() - 1 && y == grid[0].len (
            ) - 1 {
                return step;
            }
        }
        return -1;
    }
}
