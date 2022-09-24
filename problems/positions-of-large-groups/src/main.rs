fn main() {
    assert_eq!(
        Solution::large_group_positions("abbxxxxzzy".to_string()),
        vec![[3, 6]]
    );
    // assert_eq!(Solution::large_group_positions("abc".to_string()), vec![vec![]]);
    assert_eq!(
        Solution::large_group_positions("abcdddeeeeaabbbcd".to_string()),
        vec![[3, 5], [6, 9], [12, 14]]
    );
}

struct Solution {}
impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let mut prev = None;
        let mut group = 0;
        let mut ans = vec![];
        for (p, c) in s.chars().enumerate() {
            if prev == Some(c) {
                group += 1;
            } else {
                if group >= 2 {
                    ans.push(vec![p as i32 - group - 1, p as i32 - 1]);
                }
                group = 0;
            }
            prev = Some(c);
        }
        if group >= 2 {
            ans.push(vec![s.len() as i32 - group - 1, s.len() as i32 - 1]);
        }
        return ans;
    }
}
