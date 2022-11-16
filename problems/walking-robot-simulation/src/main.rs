fn main() {
    assert_eq!(Solution::robot_sim(vec![4,-1,3], vec![]), 25);
    assert_eq!(Solution::robot_sim(vec![4,-1,4,-2,4], vec![vec![2,4]]), 65);
    assert_eq!(Solution::robot_sim(vec![6,-1,-1,6], vec![]), 36);
}

struct Solution;
impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        return 0;
    }
}
