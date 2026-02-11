fn main() {
    assert_eq!(Solution::maximum_happiness_sum(vec![1, 2, 3], 2), 4);
    assert_eq!(Solution::maximum_happiness_sum(vec![1, 1, 1, 1], 2), 1);
    assert_eq!(Solution::maximum_happiness_sum(vec![2, 3, 4, 5], 1), 5);
}

struct Solution;
impl Solution {
    pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
        let mut happiness = happiness;
        happiness.sort_unstable_by(|a, b| b.cmp(a));
        happiness.iter().take(k as usize).map(|&x| x as i64).sum()
    }
}
