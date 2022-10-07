fn main() {
    assert_eq!(
        Solution::num_magic_squares_inside(vec![
            vec![4, 3, 8, 4],
            vec![9, 5, 1, 9],
            vec![2, 7, 6, 2]
        ]),
        1
    );
    assert_eq!(Solution::num_magic_squares_inside(vec![vec![8]]), 0);
}

struct Solution {}
impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        return 0;
    }
}
