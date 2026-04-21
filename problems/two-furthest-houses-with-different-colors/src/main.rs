fn main() {
    assert_eq!(Solution::max_distance(vec![1,1,1,6,1,1,1]), 3);
    assert_eq!(Solution::max_distance(vec![1, 8, 3, 8, 3]), 4);
    assert_eq!(Solution::max_distance(vec![0, 1]), 1);
}

struct Solution;
impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let mut left = 0;
        let right = colors.len() - 1;

        while colors[left] == colors[right] {
            left += 1;
        }

        (colors.len() - 1 - left).max(right) as i32
    }
}
