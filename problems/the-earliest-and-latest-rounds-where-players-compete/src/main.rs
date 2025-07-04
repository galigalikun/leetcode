fn main() {
    assert_eq!(Solution::earliest_and_latest(11, 2, 4), vec![3, 4]);
    assert_eq!(Solution::earliest_and_latest(5, 1, 5), vec![1, 1]);
}

struct Solution;
impl Solution {
    pub fn earliest_and_latest(n: i32, first_player: i32, second_player: i32) -> Vec<i32> {
        return vec![
            (first_player - 1).min(second_player - 1) + 1,
            (n - first_player).min(n - second_player) + 1,
        ];
    }
}
