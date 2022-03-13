fn main() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
}

struct Solution {}
// https://www.geeksforgeeks.org/trapping-rain-water/
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 1..height.len() {
            let mut left = height[i];
            for j in 0..i {
                left = std::cmp::max(left, height[j]);
            }
            let mut right = height[i];
            for j in i + 1..height.len() {
                right = std::cmp::max(right, height[j]);
            }
            result = result + (std::cmp::min(left, right) - height[i]);
        }
        return result;
    }
}
