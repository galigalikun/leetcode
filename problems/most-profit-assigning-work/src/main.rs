fn main() {
    assert_eq!(Solution::max_profit_assignment(vec![2,4,6,8,10], vec![10,20,30,40,50], vec![4,5,6,7]), 100);
    assert_eq!(Solution::max_profit_assignment(vec![85,47,57], vec![24,66,99], vec![40,25,25]), 0);
}

struct Solution{}
impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut jobs: Vec<(i32, i32)> = difficulty.into_iter().zip(profit).collect();
        jobs.sort_by_key(|(d, _)| *d);

        let mut workers = worker;
        workers.sort_unstable();

        let mut total_profit = 0;
        let mut best_profit_so_far = 0;
        let mut job_index = 0usize;

        for ability in workers {
            while job_index < jobs.len() && jobs[job_index].0 <= ability {
                best_profit_so_far = best_profit_so_far.max(jobs[job_index].1);
                job_index += 1;
            }

            total_profit += best_profit_so_far;
        }

        total_profit
    }
}
