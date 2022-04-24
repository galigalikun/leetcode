fn main() {
    assert_eq!(
        Solution::cal_points(vec![
            "5".to_string(),
            "2".to_string(),
            "C".to_string(),
            "D".to_string(),
            "+".to_string()
        ]),
        30
    );
    assert_eq!(
        Solution::cal_points(vec![
            "5".to_string(),
            "-2".to_string(),
            "4".to_string(),
            "C".to_string(),
            "D".to_string(),
            "9".to_string(),
            "+".to_string(),
            "+".to_string()
        ]),
        27
    );
    assert_eq!(Solution::cal_points(vec!["1".to_string()]), 1);
}

struct Solution {}
impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        for o in ops {
            println!("{}", o);
            match o.as_str() {
                "C" => {
                    if stack.len() > 0 {
                        stack.pop();
                    }
                }
                "D" => {
                    if stack.len() > 0 {
                        stack.push(stack[stack.len() - 1] * 2);
                    }
                }
                "+" => {
                    if stack.len() > 1 {
                        stack.push(stack[stack.len() - 1] + stack[stack.len() - 2]);
                    }
                }
                _ => {
                    stack.push(o.parse::<i32>().unwrap());
                }
            }
        }
        return stack.iter().fold(0, |acc, x| acc + x);
    }
}
