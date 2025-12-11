fn main() {
    assert_eq!(
        Solution::minimum_money(vec![vec![2, 1], vec![5, 0], vec![4, 2]]),
        10
    );
    assert_eq!(Solution::minimum_money(vec![vec![3, 0], vec![0, 3]]), 3);
}

struct Solution;
impl Solution {
    pub fn minimum_money(transactions: Vec<Vec<i32>>) -> i64 {
        let mut total_cost = 0i64;
        let mut max_deficit = 0i64;
        for transaction in transactions {
            let cost = transaction[0] as i64;
            let cashback = transaction[1] as i64;
            total_cost += cost;
            max_deficit = max_deficit.max(cost - cashback);
        }
        total_cost + max_deficit
    }
}
