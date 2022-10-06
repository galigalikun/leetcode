fn main() {
    assert_eq!(
        Solution::num_similar_groups(vec![
            "tars".to_string(),
            "rats".to_string(),
            "arts".to_string(),
            "star".to_string()
        ]),
        2
    );
    assert_eq!(
        Solution::num_similar_groups(vec!["omv".to_string(), "ovm".to_string()]),
        1
    );
}

struct Solution {}
impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        return 0;
    }
}
