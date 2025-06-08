fn main() {
    assert_eq!(Solution::stone_game_viii(vec![-1, 2, -3, 4, -5]), 5);
    assert_eq!(Solution::stone_game_viii(vec![7, -6, 5, 10, 5, -2, -6]), 13);
    assert_eq!(Solution::stone_game_viii(vec![-10, -12]), -22);
}

struct Solution;
impl Solution {
    pub fn stone_game_viii(stones: Vec<i32>) -> i32 {
        let mut prefix_sums: Vec<i32> = stones
            .into_iter()
            .scan(0, |sum, x| {
                *sum += x;
                Some(*sum)
            })
            .collect();
        // Remove the first element, as the game requires at least one move
        prefix_sums.remove(0);
        prefix_sums
            .into_iter()
            .rev()
            .skip(1)
            .fold((i32::MIN, 0), |(max_score, sum), x| {
                (max_score.max(sum), sum + x)
            })
            .0
    }
}
