fn main() {
    assert_eq!(
        Solution::find_lu_slength("aba".to_string(), "cdc".to_string()),
        3
    );
    assert_eq!(
        Solution::find_lu_slength("aaa".to_string(), "bbb".to_string()),
        3
    );
    assert_eq!(
        Solution::find_lu_slength("aaa".to_string(), "aaa".to_string()),
        -1
    );
}

struct Solution {}
impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        return if a != b {
            std::cmp::max(a.len(), b.len()) as i32
        } else {
            -1
        };
    }
}
