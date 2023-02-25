fn main() {
    assert_eq!(Solution::min_deletion_size(vec!["ca".to_string(),"bb".to_string(),"ac".to_string()]), 1);
    assert_eq!(Solution::min_deletion_size(vec!["xc".to_string(),"yb".to_string(),"za".to_string()]), 0);
    assert_eq!(Solution::min_deletion_size(vec!["zyx".to_string(),"wvu".to_string(),"tsr".to_string()]), 3);
}

struct Solution;
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        return 0;
    }
}
