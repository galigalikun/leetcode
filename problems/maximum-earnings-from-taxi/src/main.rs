fn main() {
    assert_eq!(
        Solution::max_taxi_earnings(5, vec![vec![2, 5, 4], vec![1, 5, 1]]),
        7
    );
    assert_eq!(
        Solution::max_taxi_earnings(
            20,
            vec![
                vec![1, 6, 1],
                vec![3, 10, 2],
                vec![10, 12, 3],
                vec![11, 12, 2],
                vec![12, 15, 2],
                vec![13, 18, 1]
            ]
        ),
        20
    );
}

struct Solution;
impl Solution {
    pub fn max_taxi_earnings(n: i32, rides: Vec<Vec<i32>>) -> i64 {
        let mut dp = vec![0; (n + 1) as usize];
        let mut rides = rides;
        rides.sort_unstable_by_key(|r| r[1]);
        let mut j = 0;
        for i in 1..=n as usize {
            dp[i] = dp[i - 1];
            while j < rides.len() && rides[j][1] as usize == i {
                let ride = &rides[j];
                dp[i] =
                    dp[i].max(dp[ride[0] as usize] + (ride[1] - ride[0]) as i64 + ride[2] as i64);
                j += 1;
            }
        }
        dp[n as usize]
    }
}
