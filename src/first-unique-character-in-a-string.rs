fn main() {
    assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
    assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
    assert_eq!(Solution::first_uniq_char("aabb".to_string()), -1);
}

pub struct Solution {}
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut i = 0;
        for c in s.chars() {
            let mut hit = false;
            let mut j = 0;
            for b in s.chars() {
                if i != j && b == c {
                    hit = true;
                    break;
                }
                j += 1;
            }
            if !hit {
                return i as i32;
            }
            i += 1;
        }
        return -1;
    }
}
