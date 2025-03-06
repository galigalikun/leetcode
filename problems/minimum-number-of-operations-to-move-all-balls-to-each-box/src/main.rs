fn main() {
    assert_eq!(Solution::min_operations("110".to_string()), vec![1, 1, 3]);
    assert_eq!(Solution::min_operations("001011".to_string()), vec![11,8,5,4,3,4]);
}

struct Solution;
impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let mut result = vec![];
        let mut left_sum = 0;
        let mut right_sum = 0;
        for i in 0..boxes.len() {
            if boxes.chars().nth(i).unwrap() == '1' {
                right_sum += i as i32;
            }
        }
        for i in 0..boxes.len() {
            if i > 0 {
                if boxes.chars().nth(i - 1).unwrap() == '1' {
                    left_sum += i as i32 - 1;
                }
            }
            if i > 0 {
                right_sum -= right_sum;
            }
            result.push(left_sum + right_sum);
        }
        return result;
    }
}
