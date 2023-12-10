fn main() {
    assert_eq!(
        Solution::freq_alphabets("10#11#12".to_string()),
        "jkab".to_string()
    );
    assert_eq!(
        Solution::freq_alphabets("1326#".to_string()),
        "acz".to_string()
    );
}

struct Solution;
impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut s = s.chars().collect::<Vec<char>>();
        let mut i = 0;
        while i < s.len() {
            if i + 2 < s.len() && s[i + 2] == '#' {
                s[i] = (s[i] as u8 - b'0' + (s[i + 1] as u8 - b'0') * 10 + b'a' - 1) as char;
                s.remove(i + 1);
                s.remove(i + 1);
            } else {
                s[i] = (s[i] as u8 - b'0' + b'a' - 1) as char;
                i += 1;
            }
        }
        return s.into_iter().collect::<String>();
    }
}
