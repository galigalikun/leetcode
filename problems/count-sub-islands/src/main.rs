fn main() {
    assert_eq!(
        Solution::count_sub_islands(
            vec![vec![1,1,1,0,0],vec![0,1,1,1,1],vec![0,0,0,0,0],vec![1,0,0,0,0],vec![1,1,0,1,1]],
            vec![vec![1,1,1,0,0],vec![0,0,1,1,1],vec![0,1,0,0,0],vec![1,0,1,1,0],vec![0,1,0,1,0]]
        ),
        3
    );
    assert_eq!(
        Solution::count_sub_islands(
            vec![vec![1,0,1,0,1],vec![1,1,1,1,1],vec![0,0,0,0,0],vec![1,1,1,1,1],vec![1,0,1,0,1]],
            vec![vec![0,0,0,0,0],vec![1,1,1,1,1],vec![0,1,0,1,0],vec![0,1,0,1,0],vec![1,0,0,0,1]]
        ),
        2
    );
}

struct Solution;
impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        return grid1
            .iter()
            .zip(grid2.iter())
            .map(|(g1, g2)| {
                g1.iter()
                    .zip(g2.iter())
                    .filter(|(g1_cell, g2_cell)| **g1_cell == 1 && **g2_cell == 1)
                    .count() as i32
            })
            .sum();
    }
}
