fn main() {
    assert_eq!(Solution::partition_labels("ababcbacadefegdehijhklij".to_string()), vec![9,7,8]);
    assert_eq!(Solution::partition_labels("eccbbbbdec".to_string()), vec![10]);
}

struct Solution{}
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        return vec![];
    }
}
