fn main() {
    assert_eq!(
        Solution::eval_rpn(vec![
            "2".to_string(),
            "1".to_string(),
            "+".to_string(),
            "3".to_string(),
            "*".to_string()
        ]),
        9
    );
    assert_eq!(
        Solution::eval_rpn(vec![
            "4".to_string(),
            "13".to_string(),
            "5".to_string(),
            "/".to_string(),
            "+".to_string()
        ]),
        6
    );
    assert_eq!(
        Solution::eval_rpn(vec![
            "10".to_string(),
            "6".to_string(),
            "9".to_string(),
            "3".to_string(),
            "+".to_string(),
            "-11".to_string(),
            "*".to_string(),
            "/".to_string(),
            "*".to_string(),
            "17".to_string(),
            "+".to_string(),
            "5".to_string(),
            "+".to_string()
        ]),
        22
    );
}

struct Solution {}
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut work: Vec<i32> = Vec::new();
        for token in tokens {
            match token.as_str() {
                "+" => {
                    if let Some(n1) = work.pop() {
                        if let Some(n2) = work.pop() {
                            work.push(n2 + n1);
                        }
                    }
                }
                "-" => {
                    if let Some(n1) = work.pop() {
                        if let Some(n2) = work.pop() {
                            work.push(n2 - n1);
                        }
                    }
                }
                "*" => {
                    if let Some(n1) = work.pop() {
                        if let Some(n2) = work.pop() {
                            work.push(n2 * n1);
                        }
                    }
                }
                "/" => {
                    if let Some(n1) = work.pop() {
                        if let Some(n2) = work.pop() {
                            work.push(n2 / n1);
                        }
                    }
                }
                num => {
                    work.push(num.parse::<i32>().unwrap());
                }
            }
        }

        return work[0];
    }
}
