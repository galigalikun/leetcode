fn main() {
    assert_eq!(Solution::orderly_queue("cba".to_string(), 1), "acb");
    assert_eq!(Solution::orderly_queue("baaca".to_string(), 3), "aaabc");
}

struct Solution;
impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        return s;
    }
}
