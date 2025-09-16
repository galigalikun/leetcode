fn main() {
    assert_eq!(
        Solution::find_different_binary_string(vec!["01".to_string(), "10".to_string()]),
        "11".to_string()
    );
    assert_eq!(
        Solution::find_different_binary_string(vec!["00".to_string(), "01".to_string(),]),
        "11".to_string()
    );
    assert_eq!(
        Solution::find_different_binary_string(vec![
            "111".to_string(),
            "011".to_string(),
            "001".to_string(),
        ]),
        "101".to_string()
    );
}

struct Solution;
impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums.len();
        let mut result = String::with_capacity(n);
        for i in 0..n {
            let bit = if nums[i].as_bytes()[i] == b'0' {
                '1'
            } else {
                '0'
            };
            result.push(bit);
        }
        result
    }
}
