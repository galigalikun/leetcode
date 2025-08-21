fn main() {
    assert_eq!(Solution::number_of_weeks(vec![1, 2, 3]), 6);
    assert_eq!(Solution::number_of_weeks(vec![5, 2, 1]), 7);
}

struct Solution;
impl Solution {
    pub fn number_of_weeks(milestones: Vec<i32>) -> i64 {
        let total: i64 = milestones.iter().map(|&x| x as i64).sum();
        let max_milestone = *milestones.iter().max().unwrap() as i64;
        let remaining = total - max_milestone;
        return if remaining >= max_milestone {
            total
        } else {
            2 * remaining + 1
        };
    }
}
