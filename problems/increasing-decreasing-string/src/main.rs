fn main() {
    assert_eq!(
        Solution::sort_string("aaaabbbbcccc".to_string()),
        "abccbaabccba"
    );
    assert_eq!(Solution::sort_string("rat".to_string()), "art");
}

struct Solution;
impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut s = s.chars().collect::<Vec<char>>();
        s.sort();
        let mut a = vec![0; 26];
        for c in &s {
            a[(*c as u8 - b'a') as usize] += 1;
        }
        let mut result = String::new();
        while result.len() < s.len() {
            for i in 0..26 {
                if a[i] > 0 {
                    result.push((i as u8 + b'a') as char);
                    a[i] -= 1;
                }
            }
            for i in (0..26).rev() {
                if a[i] > 0 {
                    result.push((i as u8 + b'a') as char);
                    a[i] -= 1;
                }
            }
        }
        return result;
    }
}
