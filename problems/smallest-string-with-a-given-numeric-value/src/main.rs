use std::vec;

fn main() {
    assert_eq!(Solution::get_smallest_string(3, 27), "aay");
    assert_eq!(Solution::get_smallest_string(5, 73), "aaszz");
}

struct Solution;
impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        let mut s = String::new();
        let mut k = k;
        for i in (1..=n).rev() {
            let c = std::cmp::min(k - i + 1, 26);
            s.push((b'a' + c as u8 - 1) as char);
            k -= c;
        }
        s.chars().rev().collect()
    }
}
