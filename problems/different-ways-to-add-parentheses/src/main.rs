fn main() {
    assert_eq!(
        Solution::diff_ways_to_compute("2-1-1".to_string()),
        vec![0, 2]
    );

    assert_eq!(
        Solution::diff_ways_to_compute("2*3-4*5".to_string()),
        vec![10, -10, -14, -10, -34]
    );
}

struct Solution {}
impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut result = vec![];
        for i in (0..expression.len()).rev() {
            match expression.chars().nth(i) {
                Some('+') | Some('-') | Some('*') => {
                    let left = &expression[0..i];
                    let right = &expression[i + 1..];
                    let left_res = Solution::diff_ways_to_compute(left.to_string());
                    let right_res = Solution::diff_ways_to_compute(right.to_string());
                    for left_ele in left_res {
                        for right_ele in &right_res {
                            let mut tmp = 0;
                            match expression.chars().nth(i) {
                                Some('+') => {
                                    tmp = left_ele + right_ele;
                                }
                                Some('-') => {
                                    tmp = left_ele - right_ele;
                                }
                                Some('*') => {
                                    tmp = left_ele * right_ele;
                                }
                                _ => {}
                            }
                            result.push(tmp);
                        }
                    }
                }
                _ => {}
            }
        }
        if result.len() == 0 {
            result.push(expression.parse::<i32>().unwrap());
        }
        return result;
    }
}
