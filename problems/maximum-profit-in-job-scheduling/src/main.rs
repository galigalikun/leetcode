fn main() {
    assert_eq!(
        Solution::job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]),
        120
    );
    assert_eq!(
        Solution::job_scheduling(
            vec![1, 2, 3, 4, 6],
            vec![3, 5, 10, 6, 9],
            vec![20, 20, 100, 70, 60]
        ),
        150
    );
    assert_eq!(
        Solution::job_scheduling(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4]),
        6
    );
}

struct Solution;
impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs: Vec<(i32, i32, i32)> = start_time
            .iter()
            .zip(end_time.iter())
            .zip(profit.iter())
            .map(|((s, e), p)| (*s, *e, *p))
            .collect();
        jobs.sort_by(|a, b| a.1.cmp(&b.1));
        let mut dp: Vec<i32> = vec![0; jobs.len()];
        dp[0] = jobs[0].2;
        for i in 1..jobs.len() {
            let mut l = 0;
            let mut r = i - 1;
            while l < r {
                let m = (l + r + 1) / 2;
                if jobs[m].1 <= jobs[i].0 {
                    l = m;
                } else {
                    r = m - 1;
                }
            }
            dp[i] = std::cmp::max(dp[i - 1], dp[l] + jobs[i].2);
        }
        return dp[jobs.len() - 1];
    }
}
