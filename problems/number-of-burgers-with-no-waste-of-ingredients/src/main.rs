use std::vec;

fn main() {
    assert_eq!(Solution::num_of_burgers(16, 7), vec![1, 6]);
    assert_eq!(Solution::num_of_burgers(17, 4), vec![]);
    assert_eq!(Solution::num_of_burgers(4, 17), vec![]);
}

struct Solution;
impl Solution {
    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        for x in 0..=tomato_slices {
            let y = (tomato_slices - x * 2) / 4;
            if x + y == cheese_slices {
                return vec![x, y];
            }
        }
        return vec![];
    }
}
