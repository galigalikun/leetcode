fn main() {
    assert_eq!(
        Solution::count_negatives(vec![
            vec![4, 3, 2, -1],
            vec![3, 2, 1, -1],
            vec![1, 1, -1, -2],
            vec![-1, -1, -2, -3]
        ]),
        8
    );
    assert_eq!(Solution::count_negatives(vec![vec![3, 2], vec![1, 0]]), 0);
}

struct Solution;
impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        return grid.iter().fold(0, |acc, row| {
            return acc + row.iter().filter(|&x| *x < 0).count() as i32;
        });
    }
}
