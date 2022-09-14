fn main() {
    assert_eq!(
        Solution::shortest_to_char("loveleetcode".to_string(), 'e'),
        vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
    );
    assert_eq!(
        Solution::shortest_to_char("aaab".to_string(), 'b'),
        vec![3, 2, 1, 0]
    );
}

struct Solution {}
impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut ans = vec![-1; s.len()];

        for (i, p) in s.chars().enumerate() {
            if p == c {
                ans[i] = 0;
            } else if let Some(p) = s[i..].chars().position(|v| v == c) {
                ans[i] = p as i32;
            }
        }
        return ans;
    }
}
