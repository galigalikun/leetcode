fn main() {
    assert_eq!(
        Solution::min_operations(vec![
            "d1/".to_string(),
            "d2/".to_string(),
            "../".to_string(),
            "d21/".to_string(),
            "./".to_string()
        ]),
        2
    );
    assert_eq!(
        Solution::min_operations(vec![
            "d1/".to_string(),
            "d2/".to_string(),
            "./".to_string(),
            "d3/".to_string(),
            "../".to_string(),
            "d31/".to_string()
        ]),
        3
    );
    assert_eq!(
        Solution::min_operations(vec![
            "d1/".to_string(),
            "../".to_string(),
            "../".to_string(),
            "../".to_string()
        ]),
        0
    );
}

struct Solution;
impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        for log in logs {
            if log == "./" {
                continue;
            } else if log == "../" {
                stack.pop();
            } else {
                stack.push(log);
            }
        }
        stack.len() as i32
    }
}
