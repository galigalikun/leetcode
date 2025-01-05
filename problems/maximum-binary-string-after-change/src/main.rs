fn main() {
    assert_eq!(Solution::maximum_binary_string("000110".to_string()), "111011".to_string());
    assert_eq!(Solution::maximum_binary_string("01".to_string()), "01".to_string());
}

struct Solution;
impl Solution {
    pub fn maximum_binary_string(binary: String) -> String {
        return binary;
    }
}
