fn main() {
    assert_eq!(
        Solution::di_string_match("IDID".to_string()),
        vec![0, 4, 1, 3, 2]
    );
    assert_eq!(
        Solution::di_string_match("III".to_string()),
        vec![0, 1, 2, 3]
    );
    assert_eq!(
        Solution::di_string_match("DDI".to_string()),
        vec![3, 2, 0, 1]
    );
}

struct Solution;
impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut perm = vec![0; s.len() + 1];
        let mut I = 0;
        let mut D = s.len() as i32;
        for (i, c) in s.chars().enumerate() {
            if c == 'I' {
                perm[i] = I;
                I += 1;
            } else if c == 'D' {
                perm[i] = D;
                D -= 1;
            }
        }
        perm[s.len()] = D;
        return perm;
    }
}
