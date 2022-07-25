fn main() {
    assert_eq!(Solution::basic_calculator_iv("e + 8 - a + 5".to_string(), vec!["e".to_string()], vec![1]), vec!["-1*a","14"]);
    assert_eq!(Solution::basic_calculator_iv("e - 8 + temperature - pressure".to_string(), vec!["e".to_string(), "temperature".to_string()], vec![1, 12]), vec!["-1*pressure","5"]);
    assert_eq!(Solution::basic_calculator_iv("(e + 8) * (e - 8)".to_string(), vec![], vec![]), vec!["1*e*e","-64"]);
}

struct Solution{}
impl Solution {
    pub fn basic_calculator_iv(expression: String, evalvars: Vec<String>, evalints: Vec<i32>) -> Vec<String> {
        return vec![];
    }
}
