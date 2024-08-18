fn main() {
    assert_eq!(Solution::three_consecutive_odds(vec![2, 6, 4, 1]), false);
    assert_eq!(
        Solution::three_consecutive_odds(vec![1, 2, 34, 3, 4, 5, 7, 23, 12]),
        true
    );
}

struct Solution;
impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        return arr.windows(3).any(|x| x.iter().all(|&y| y % 2 != 0));
    }
}
