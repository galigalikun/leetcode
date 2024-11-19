fn main() {
    assert_eq!(Solution::minimum_deletions("aababbab".to_string()), 2);
    assert_eq!(Solution::minimum_deletions("bbaaaaabb".to_string()), 2);
}

struct Solution;
impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut a = 0;
        let mut b = 0;
        for c in s.chars() {
            if c == 'a' {
                a += 1;
            } else {
                b = b.min(a + 1);
            }
        }
        b = b.min(a);
        return b as i32;
    }
}
