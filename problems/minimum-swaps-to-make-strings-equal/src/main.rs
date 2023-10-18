fn main() {
    assert_eq!(
        Solution::minimum_swap("xx".to_string(), "yy".to_string()),
        1
    );
    assert_eq!(
        Solution::minimum_swap("xy".to_string(), "yx".to_string()),
        2
    );
    assert_eq!(
        Solution::minimum_swap("xx".to_string(), "xy".to_string()),
        -1
    );
}

struct Solution;
impl Solution {
    pub fn minimum_swap(s1: String, s2: String) -> i32 {
        let mut x = 0;
        let mut y = 0;
        let mut i = 0;
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        while i < s1.len() {
            if s1[i] != s2[i] {
                if s1[i] == b'x' {
                    x += 1;
                } else {
                    y += 1;
                }
            }
            i += 1;
        }
        if (x + y) % 2 == 0 {
            return x / 2 + y / 2 + x % 2 + y % 2;
        }
        return -1;
    }
}
