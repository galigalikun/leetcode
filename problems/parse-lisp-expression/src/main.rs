fn main() {
    assert_eq!(
        Solution::evaluate("(let x 2 (mult x (let x 3 y 4 (add x y))))".to_string()),
        14
    );
    assert_eq!(Solution::evaluate("(let x 3 x 2 x)".to_string()), 2);
    assert_eq!(
        Solution::evaluate("(let x 1 y 2 x (add x y) (add x y))".to_string()),
        3
    );
}

struct Solution {}
impl Solution {
    pub fn evaluate(expression: String) -> i32 {
        return 0;
    }
}
