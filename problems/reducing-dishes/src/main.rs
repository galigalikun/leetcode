use std::vec;

fn main() {
    assert_eq!(Solution::max_satisfaction(vec![-1, -8, 0, 5, -9]), 14);
    assert_eq!(Solution::max_satisfaction(vec![4, 3, 2]), 20);
    assert_eq!(Solution::max_satisfaction(vec![-1, -4, -5]), 0);
}

struct Solution;
impl Solution {
    pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        let mut satisfaction = satisfaction;
        satisfaction.sort();
        let mut sum = 0;
        let mut total = 0;
        for i in 0..satisfaction.len() {
            sum += satisfaction[i];
            total += sum;
        }
        if total > 0 {
            return total;
        }
        return 0;
    }
}
