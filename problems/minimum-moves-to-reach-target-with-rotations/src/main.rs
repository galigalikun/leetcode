fn main() {
    assert_eq!(
        Solution::minimum_moves(vec![
            vec![0, 0, 0, 0, 0, 1],
            vec![1, 1, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 1, 1],
            vec![0, 0, 1, 0, 1, 0],
            vec![0, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 0, 0]
        ]),
        11
    );
    assert_eq!(
        Solution::minimum_moves(vec![
            vec![0, 0, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 1, 1],
            vec![1, 1, 0, 0, 0, 1],
            vec![1, 1, 1, 0, 0, 1],
            vec![1, 1, 1, 0, 0, 1],
            vec![1, 1, 1, 0, 0, 0]
        ]),
        9
    );
}

struct Solution;
impl Solution {
    fn bfs(
        grid: &mut Vec<Vec<i32>>,
        queue: &mut std::collections::VecDeque<((i32, i32), (i32, i32))>,
        visited: &mut std::collections::HashSet<((usize, usize), (usize, usize))>,
        step: &mut i32,
        end: &mut (usize, usize),
    ) -> i32 {
        return 0;
    }
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut queue = std::collections::VecDeque::new();
        let mut visited = std::collections::HashSet::new();
        let mut step = 0;
        let mut end = (grid.len() - 1, grid.len() - 2);
        queue.push_back(((0, 0), (0, 1)));
        visited.insert(((0, 0), (0, 1)));
        for i in 0..grid.len() {
            for j in 0..grid.len() {
                if grid[i][j] == 1 {
                    visited.insert(((i, j), (i, j)));
                }
            }
        }
        return Self::bfs(&mut grid, &mut queue, &mut visited, &mut step, &mut end);
    }
}
