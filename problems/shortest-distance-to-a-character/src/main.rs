fn main() {
    assert_eq!(
        Solution::shortest_to_char("loveleetcode".to_string(), 'e'),
        vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
    );
    assert_eq!(
        Solution::shortest_to_char("aaab".to_string(), 'b'),
        vec![3, 2, 1, 0]
    );
    assert_eq!(
        Solution::shortest_to_char("aaba".to_string(), 'b'),
        vec![2, 1, 0, 1]
    );
}

struct Solution {}
impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut ans = vec![-1; s.len()];
        let mut idx = None;
        for (i, p) in s.chars().enumerate() {
            if p == c {
                ans[i] = 0;
                idx = Some(i);
            } else if let Some(p) = s[i..].chars().position(|v| v == c) {
                if let Some(id) = idx {
                    ans[i] = std::cmp::min(i - id, p) as i32;
                } else {
                    ans[i] = p as i32;
                }
            } else if let Some(id) = idx {
                ans[i] = (i - id) as i32;
            }
        }
        return ans;
    }
}
