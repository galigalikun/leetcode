fn main() {
    assert_eq!(
        Solution::num_special_equiv_groups(vec![
            "abcd".to_string(),
            "cdab".to_string(),
            "cbad".to_string(),
            "xyzz".to_string(),
            "zzxy".to_string(),
            "zzyx".to_string()
        ]),
        3
    );
    assert_eq!(
        Solution::num_special_equiv_groups(vec![
            "abc".to_string(),
            "acb".to_string(),
            "bac".to_string(),
            "bca".to_string(),
            "cab".to_string(),
            "cba".to_string()
        ]),
        3
    );
}

struct Solution;
impl Solution {
    pub fn num_special_equiv_groups(words: Vec<String>) -> i32 {
        return 0;
    }
}
