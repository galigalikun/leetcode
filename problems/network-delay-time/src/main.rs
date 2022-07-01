fn main() {
    assert_eq!(
        Solution::network_delay_time(vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]], 4, 2),
        2
    );
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1]], 2, 1), 1);
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1]], 2, 2), -1);
}

struct Solution {}
impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        return 0;
    }
}
