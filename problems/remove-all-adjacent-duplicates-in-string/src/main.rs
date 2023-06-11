fn main() {
    assert_eq!(Solution::remove_duplicates("abbaca".to_string()), "ca");
    assert_eq!(Solution::remove_duplicates("azxxzy".to_string()), "ay");
}

struct Solution;
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut s = s;
        let mut stack = Vec::new();
        for c in s.chars() {
            if let Some(&last) = stack.last() {
                if last == c {
                    stack.pop();
                    continue;
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
