use std::vec;

fn main() {
    assert_eq!(
        Solution::kids_with_candies(vec![2, 3, 5, 1, 3], 3),
        vec![true, true, true, false, true]
    );
    assert_eq!(
        Solution::kids_with_candies(vec![4, 2, 1, 1, 2], 1),
        vec![true, false, false, false, false]
    );
    assert_eq!(
        Solution::kids_with_candies(vec![12, 1, 12], 10),
        vec![true, false, true]
    );
}

struct Solution;
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        return candies
            .iter()
            .map(|candy| *candy + extra_candies >= *candies.iter().max().unwrap())
            .collect();
    }
}
