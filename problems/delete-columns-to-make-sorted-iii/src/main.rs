fn main() {
    assert_eq!(Solution::min_deletion_size(vec!["babca".to_string(),"bbazb".to_string()]), 3);
    assert_eq!(Solution::min_deletion_size(vec!["edcba".to_string()]), 4);
    assert_eq!(Solution::min_deletion_size(vec!["ghi".to_string(),"def".to_string(),"abc".to_string()]), 0);
}

struct Solution;
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        return 0;
    }
}
