fn main() {
    assert_eq!(
        Solution::color_border(vec![vec![1, 1], vec![1, 2]], 0, 0, 3),
        vec![[3, 3], [3, 2]]
    );
    assert_eq!(
        Solution::color_border(vec![vec![1, 2, 2], vec![2, 3, 2]], 0, 1, 3),
        vec![[1, 3, 3], [2, 3, 3]]
    );
    assert_eq!(
        Solution::color_border(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]], 1, 1, 2),
        vec![[2, 2, 2], [2, 1, 2], [2, 2, 2]]
    );
}

struct Solution;
impl Solution {
    pub fn color_border(grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
        let mut grid = grid;
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let mut queue = vec![(row as usize, col as usize)];
        let mut border = vec![];
        return vec![];
    }
}
