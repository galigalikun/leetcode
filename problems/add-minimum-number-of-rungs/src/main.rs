fn main() {
    assert_eq!(Solution::add_rungs(vec![1, 3, 5, 10], 2), 2);
    assert_eq!(Solution::add_rungs(vec![3, 6, 8, 10], 3), 0);
    assert_eq!(Solution::add_rungs(vec![3, 4, 6, 7], 2), 1);
}

struct Solution;
impl Solution {
    pub fn add_rungs(rungs: Vec<i32>, dist: i32) -> i32 {
        return rungs
            .windows(2)
            .fold(0, |acc, w| acc + ((w[1] - w[0] - 1) / dist))
            + if rungs.is_empty() {
                0
            } else {
                (rungs[0] - 1) / dist
            };
    }
}
