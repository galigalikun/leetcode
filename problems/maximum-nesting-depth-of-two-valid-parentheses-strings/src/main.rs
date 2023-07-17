fn main() {
    assert_eq!(
        Solution::max_depth_after_split("(()())".to_string()),
        vec![0, 1, 1, 1, 1, 0]
    );
    assert_eq!(
        Solution::max_depth_after_split("()(())()".to_string()),
        vec![0, 0, 0, 1, 1, 0, 1, 1]
    );
}

struct Solution;
impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut depth = 0;
        let mut result = vec![];
        for c in seq.chars() {
            if c == '(' {
                depth += 1;
                result.push(depth % 2);
            } else {
                result.push(depth % 2);
                depth -= 1;
            }
        }
        return result;
    }
}
