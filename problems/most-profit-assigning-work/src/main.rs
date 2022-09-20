fn main() {
    assert_eq!(Solution::max_profit_assignment(vec![2,4,6,8,10], vec![10,20,30,40,50], vec![4,5,6,7]), 100);
    assert_eq!(Solution::max_profit_assignment(vec![85,47,57], vec![24,66,99], vec![40,25,25]), 0);
}

struct Solution{}
impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        return 0;
    }
}
