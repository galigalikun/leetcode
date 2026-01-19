fn main() {
    assert_eq!(Solution::max_run_time(2, vec![3, 3, 3]), 4);
    assert_eq!(Solution::max_run_time(2, vec![1, 1, 1, 1]), 2);
    assert_eq!(Solution::max_run_time(3, vec![10, 10, 3, 5]), 8);
}

struct Solution;
impl Solution {
    pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
        return batteries.iter().sum::<i32>() as i64 / n as i64;
    }
}
