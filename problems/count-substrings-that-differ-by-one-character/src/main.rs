fn main() {
    assert_eq!(
        Solution::count_substrings("aba".to_string(), "baba".to_string()),
        6
    );
    assert_eq!(
        Solution::count_substrings("ab".to_string(), "bb".to_string()),
        3
    );
}

struct Solution;
impl Solution {
    pub fn count_substrings(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut ans = 0;
        for i in 0..s.len() {
            for j in 0..t.len() {
                let mut diff = 0;
                for k in 0..s.len().max(t.len()) {
                    if i + k >= s.len() || j + k >= t.len() {
                        break;
                    }
                    if s[i + k] != t[j + k] {
                        diff += 1;
                    }
                    if diff == 1 {
                        ans += 1;
                    }
                    if diff > 1 {
                        break;
                    }
                }
            }
        }
        ans
    }
}
