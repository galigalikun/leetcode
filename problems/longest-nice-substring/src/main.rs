fn main() {
    assert_eq!(
        Solution::longest_nice_substring("YazaAay".to_string()),
        "aAa"
    );
    assert_eq!(Solution::longest_nice_substring("Bb".to_string()), "Bb");
    assert_eq!(Solution::longest_nice_substring("c".to_string()), "");
}

struct Solution;
impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        let s = s.as_bytes();
        let n = s.len();
        let mut res = String::new();
        for i in 0..n {
            for j in (i + 1)..n {
                if j - i + 1 <= res.len() {
                    continue;
                }
                let mut is_nice = true;
                let mut set = [false; 26];
                for k in i..=j {
                    let c = s[k];
                    if c >= b'a' && c <= b'z' {
                        set[(c - b'a') as usize] = true;
                    } else {
                        set[(c - b'A') as usize] = true;
                    }
                }
                for k in i..=j {
                    let c = s[k];
                    if c >= b'a' && c <= b'z' {
                        if !set[(c - b'a') as usize] && set[(c - b'A') as usize] {
                            is_nice = false;
                            break;
                        }
                    } else {
                        if !set[(c - b'A') as usize] && set[(c - b'a') as usize] {
                            is_nice = false;
                            break;
                        }
                    }
                }
                if is_nice {
                    res = std::str::from_utf8(&s[i..=j]).unwrap().to_string();
                }
            }
        }
        res
    }
}
