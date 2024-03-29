fn main() {
    assert_eq!(Solution::height_checker(vec![1, 1, 4, 2, 1, 3]), 3);
    assert_eq!(Solution::height_checker(vec![5, 1, 2, 3, 4]), 5);
    assert_eq!(Solution::height_checker(vec![1, 2, 3, 4, 5]), 0);
}

struct Solution;
impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut sorted = heights.clone();
        sorted.sort();
        let mut count = 0;
        for i in 0..heights.len() {
            if heights[i] != sorted[i] {
                count += 1;
            }
        }
        return count;
    }
}
