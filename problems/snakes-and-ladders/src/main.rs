fn main() {
    assert_eq!(Solution::snakes_and_ladders(vec![vec![-1,-1,-1,-1,-1,-1],vec![-1,-1,-1,-1,-1,-1],vec![-1,-1,-1,-1,-1,-1],vec![-1,35,-1,-1,13,-1],vec![-1,-1,-1,-1,-1,-1],vec![-1,15,-1,-1,-1,-1]]), 4);
    assert_eq!(Solution::snakes_and_ladders(vec![vec![-1,-1],vec![-1,3]]), 1);
}

struct Solution;
impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        return 0;
    }
}
