fn main() {
    assert_eq!(Solution::bag_of_tokens_score(vec![100], 50), 0);
    assert_eq!(Solution::bag_of_tokens_score(vec![100,200], 150), 1);
    assert_eq!(Solution::bag_of_tokens_score(vec![100,200,300,400], 200), 2);
}

struct Solution;
impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {
        return 0;
    }
}
