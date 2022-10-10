fn main() {
    assert_eq!(
        Solution::split_into_fibonacci("1101111".to_string()),
        vec![11, 0, 11, 11]
    );
    assert_eq!(
        Solution::split_into_fibonacci("112358130".to_string()),
        vec![]
    );
    assert_eq!(Solution::split_into_fibonacci("0123".to_string()), vec![]);
}

struct Solution {}
impl Solution {
    pub fn split_into_fibonacci(num: String) -> Vec<i32> {
        return vec![];
    }
}
