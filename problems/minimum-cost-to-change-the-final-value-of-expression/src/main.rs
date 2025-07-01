fn main() {
    assert_eq!(Solution::min_operations_to_flip("1&(0|1)".to_string()), 1);
    assert_eq!(
        Solution::min_operations_to_flip("(0&0)&(0&0&0)".to_string()),
        3
    );
    assert_eq!(
        Solution::min_operations_to_flip("(0|(1|0&1))".to_string()),
        1
    );
}

struct Solution;
impl Solution {
    fn min_operations_to_flip_helper(expression: &str, start: usize, end: usize) -> i128 {
        if start == end {
            return if expression.chars().nth(start).unwrap() == '0' {
                1
            } else {
                0
            };
        }

        let mut min_operations = i128::MAX;

        for i in start..end {
            if expression.chars().nth(i).unwrap() == '&'
                || expression.chars().nth(i).unwrap() == '|'
            {
                let left_result = Self::min_operations_to_flip_helper(expression, start, i - 1);
                let right_result = Self::min_operations_to_flip_helper(expression, i + 1, end);

                let current_operations = match expression.chars().nth(i).unwrap() {
                    '&' => left_result + right_result,
                    '|' => left_result + right_result,
                    _ => continue,
                };

                min_operations = min_operations.min(current_operations);
            }
        }

        min_operations
    }
    pub fn min_operations_to_flip(expression: String) -> i32 {
        return Self::min_operations_to_flip_helper(&expression, 0, expression.len() - 1) as i32;
    }
}
