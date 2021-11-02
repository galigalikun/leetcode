fn main() {
    assert_eq!(Solution::min_moves2(vec![1, 2, 3]), 2);
    assert_eq!(Solution::min_moves2(vec![1, 10, 2, 9]), 16);
    assert_eq!(Solution::min_moves2(vec![1, 1, 2]), 1);
    assert_eq!(Solution::min_moves2(vec![1, 0, 0, 8, 6]), 14);
}

pub struct Solution {}
impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mut nn = nums.clone();
        nn.sort();
        let idx = nn.len() / 2;
        let median = if nn.len() % 2 == 0 {
            (nn[idx - 1] + nn[idx]) as f32 / 2.0
        } else {
            nn[idx] as f32
        }
        .ceil() as i32;

        return nums.iter().fold(0, |sum, a| sum + (median - a).abs());
    }
}
