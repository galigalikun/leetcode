fn main() {
    assert_eq!(Solution::order_of_largest_plus_sign(5, vec![vec![4,2]]), 2);
    assert_eq!(Solution::order_of_largest_plus_sign(0, vec![vec![0,0]]), 0);
}

struct Solution{}
impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let mut grid = vec![vec![1;n as usize];n as usize];
        for mine in mines {
            grid[mine[0] as usize][mine[1] as usize] = 0;
        }
        return 0;
    }
}
