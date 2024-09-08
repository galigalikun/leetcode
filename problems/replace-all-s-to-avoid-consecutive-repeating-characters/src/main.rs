fn main() {
    assert_eq!(
        Solution::modify_string("?zs".to_string()),
        "azs".to_string()
    );
    assert_eq!(
        Solution::modify_string("ubv?w".to_string()),
        "ubvaw".to_string()
    );
}

struct Solution;
impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut s = s.into_bytes();
        for i in 0..s.len() {
            if s[i] == b'?' {
                for c in b'a'..=b'z' {
                    if i > 0 && s[i - 1] == c {
                        continue;
                    }
                    if i + 1 < s.len() && s[i + 1] == c {
                        continue;
                    }
                    s[i] = c;
                    break;
                }
            }
        }
        String::from_utf8(s).unwrap()
    }
}
