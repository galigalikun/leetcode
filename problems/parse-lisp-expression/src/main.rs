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
        for c in expression.chars() {
            match c {
                '(' => {}
                ')' => {}
                ' ' => {}
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    let num = c as i32 - 48;
                    println!("debug {}", num);
                }
                _ => {}
            }
        }
        return 0;
    }
}
