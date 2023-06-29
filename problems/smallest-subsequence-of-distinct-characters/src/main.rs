fn main() {
    assert_eq!(Solution::smallest_subsequence("bcabc".to_string()), "abc");
    assert_eq!(
        Solution::smallest_subsequence("cbacdcbc".to_string()),
        "acdb"
    );
}

struct Solution;
impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let mut s = s;
        let mut stack = vec![];
        let mut last = vec![0; 26];
        for (i, c) in s.chars().enumerate() {
            last[(c as u8 - b'a') as usize] = i;
        }
        for (i, c) in s.chars().enumerate() {
            if stack.contains(&c) {
                continue;
            }
            while let Some(&top) = stack.last() {
                if top > c && last[(top as u8 - b'a') as usize] > i {
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(c);
        }
        s.clear();
        for c in stack {
            s.push(c);
        }

        return s;
    }
}
