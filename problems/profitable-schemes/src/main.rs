fn main() {
    assert_eq!(Solution::profitable_schemes(5, 3, vec![2,2], vec![2,3]), 2);
    assert_eq!(Solution::profitable_schemes(10, 5, vec![2,3,5], vec![6,7,8]), 7);
}

struct Solution;
impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        return -1;
    }
}
