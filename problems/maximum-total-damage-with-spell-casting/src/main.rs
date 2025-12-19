fn main() {
    assert_eq!(Solution::maximum_total_damage(vec![1, 1, 3, 4]), 6);
    assert_eq!(Solution::maximum_total_damage(vec![7, 1, 6, 6]), 13);
}

struct Solution;
impl Solution {
    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        return power.iter().map(|&x| x as i64).sum();
    }
}
