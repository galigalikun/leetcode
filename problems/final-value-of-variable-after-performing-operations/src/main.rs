fn main() {
    assert_eq!(Solution::final_value_after_operations(vec!["--X".to_string(),"X++".to_string(),"X++".to_string()]), 1);
    assert_eq!(Solution::final_value_after_operations(vec!["++X".to_string(),"++X".to_string(),"X++".to_string()]), 3);
    assert_eq!(Solution::final_value_after_operations(vec!["X++".to_string(),"++X".to_string(),"--X".to_string(),"X--".to_string()]), 0);
}

struct Solution;
impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        operations.iter().fold(0, |acc, op| {
            if op.contains('+') {
                acc + 1
            } else {
                acc - 1
            }
        })
    }
}
