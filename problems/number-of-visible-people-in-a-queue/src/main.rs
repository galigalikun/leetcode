fn main() {
    assert_eq!(
        Solution::can_see_persons_count(vec![10, 6, 8, 5, 11, 9]),
        vec![3, 1, 2, 1, 1, 0]
    );
    assert_eq!(
        Solution::can_see_persons_count(vec![5, 1, 2, 3, 10]),
        vec![4, 1, 1, 1, 0]
    );
}

struct Solution;
impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let n = heights.len();
        let mut result = vec![0; n];
        let mut stack = Vec::new();

        for i in 0..n {
            while let Some(&j) = stack.last() {
                if heights[j] < heights[i] {
                    result[j] += 1;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i);
        }

        result
    }
}
