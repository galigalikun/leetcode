fn main() {
    assert_eq!(Solution::reverse_str("abcdefg".to_string(), 2), "bacdfeg");
    assert_eq!(Solution::reverse_str("abcd".to_string(), 2), "bacd");
    assert_eq!(Solution::reverse_str("abcdefg".to_string(), 8), "gfedcba");
    assert_eq!(Solution::reverse_str("abcdefg".to_string(), 4), "dcbaefg");
}

struct Solution {}
impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut b = 0;
        let mut ans = String::new();
        if k as usize > s.len() {
            ans = s.chars().rev().collect::<String>();
        } else {
            for i in (0..s.len()).step_by(k as usize) {
                let n = i + k as usize;
                ans = format!(
                    "{}{}",
                    ans,
                    if n > s.len() {
                        if b % 2 == 0 {
                            format!("{}", &s[i..].chars().rev().collect::<String>())
                        } else {
                            format!("{}", &s[i..])
                        }
                    } else if b % 2 == 0 {
                        format!("{}", &s[i..n].chars().rev().collect::<String>())
                    } else {
                        format!("{}", &s[i..n])
                    }
                );
                b += 1;
            }
        }

        return ans;
    }
}
