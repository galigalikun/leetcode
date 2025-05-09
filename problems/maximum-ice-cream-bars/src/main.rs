fn main() {
    assert_eq!(Solution::max_ice_cream(vec![1, 3, 2, 4, 1], 7), 4);
    assert_eq!(Solution::max_ice_cream(vec![10, 6, 8, 7, 7, 8], 5), 0);
    assert_eq!(Solution::max_ice_cream(vec![1, 6, 3, 1, 2, 5], 20), 6);
}

struct Solution;
impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        let mut costs = costs;
        costs.sort();
        let mut count = 0;
        let mut total_cost = 0;

        for cost in costs {
            if total_cost + cost <= coins {
                total_cost += cost;
                count += 1;
            } else {
                break;
            }
        }

        count
    }
}
