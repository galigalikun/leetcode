fn main() {
    assert_eq!(Solution::min_number_of_seconds(4, vec![2, 1, 1]), 3);
    assert_eq!(Solution::min_number_of_seconds(10, vec![3, 2, 2, 4]), 12);
    assert_eq!(Solution::min_number_of_seconds(5, vec![1]), 15);
}

struct Solution;
impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let mut worker_times = worker_times;
        worker_times.sort_unstable();
        let mut ans = 0;
        for (i, &t) in worker_times.iter().enumerate() {
            ans = ans.max((mountain_height as i64 + i as i64) * t as i64);
        }
        ans
    }
}
