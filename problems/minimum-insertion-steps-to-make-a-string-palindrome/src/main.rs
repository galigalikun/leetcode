fn main() {
    assert_eq!(Solution::min_insertions("zzazz".to_string()), 0);
    assert_eq!(Solution::min_insertions("mbadm".to_string()), 2);
    assert_eq!(Solution::min_insertions("leetcode".to_string()), 5);
}

struct Solution;
impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut s = s.chars().collect::<Vec<char>>();
        let mut i = 0;
        let mut j = 0;
        while i < s.len() {
            if s[i] == '(' {
                j += 1;
            } else {
                if j == 0 {
                    s.insert(i, '(');
                    i += 1;
                } else {
                    j -= 1;
                }
                if i + 1 < s.len() && s[i + 1] == ')' {
                    i += 1;
                } else {
                    s.insert(i + 1, ')');
                }
            }
            i += 1;
        }
        return j as i32 + s.len() as i32;
    }
}
