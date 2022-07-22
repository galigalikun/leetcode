fn main() {
    assert_eq!(Solution::reorganize_string("aab".to_string()), "aba");
    assert_eq!(Solution::reorganize_string("aaab".to_string()), "");
}

struct Solution{}
impl Solution {
    pub fn reorganize_string(s: String) -> String {
        return "".to_string();
    }
}
