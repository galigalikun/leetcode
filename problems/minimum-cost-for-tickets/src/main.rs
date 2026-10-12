fn main() {
    assert_eq!(Solution::mincost_tickets(vec![1,4,6,7,8,20], vec![2,7,15]), 11);
    assert_eq!(Solution::mincost_tickets(vec![1,2,3,4,5,6,7,8,9,10,30,31], vec![2,7,15]), 17);
}

struct Solution;
impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let last_day = *days.last().unwrap() as usize;
        let mut is_travel_day = vec![false; last_day + 1];
        for day in days {
            is_travel_day[day as usize] = true;
        }

        let mut dp = vec![0; last_day + 1];
        for day in 1..=last_day {
            if !is_travel_day[day] {
                dp[day] = dp[day - 1];
                continue;
            }

            let one_day = dp[day - 1] + costs[0];
            let seven_day = dp[day.saturating_sub(7)] + costs[1];
            let thirty_day = dp[day.saturating_sub(30)] + costs[2];
            dp[day] = one_day.min(seven_day).min(thirty_day);
        }

        dp[last_day]
    }
}
