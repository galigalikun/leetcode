fn main() {
    assert_eq!(Solution::moves_to_stamp("abc".to_string(), "ababc".to_string()), vec![0,2]);
    assert_eq!(Solution::moves_to_stamp("abca".to_string(), "aabcaca".to_string()), vec![3,0,1]);
}

struct Solution;
impl Solution {
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        return vec![];
    }
}
