fn main() {
    assert_eq!(Solution::longest_dup_substring("banana".to_string()), "ana");
    assert_eq!(Solution::longest_dup_substring("abcd".to_string()), "");
}

struct Solution;
impl Solution {
    fn check(s: &String, len: usize) -> bool {
        let mut map = std::collections::HashMap::new();
        let mut hash = 0;
        let mut base = 1;
        for i in 0..len {
            hash = hash * 26 + (s.as_bytes()[i] - b'a') as i64;
            base *= 26;
        }
        map.insert(hash, 0);
        for i in 1..=s.len() - len {
            hash = hash * 26 - base * (s.as_bytes()[i - 1] - b'a') as i64
                + (s.as_bytes()[i + len - 1] - b'a') as i64;
            if map.contains_key(&hash) {
                let j = map[&hash];
                if s[j..j + len] == s[i..i + len] {
                    return true;
                }
            }
            map.insert(hash, i);
        }
        return false;
    }
    pub fn longest_dup_substring(s: String) -> String {
        let mut s = s;
        let mut l = 0;
        let mut r = s.len() - 1;
        while l < r {
            let mid = (l + r + 1) / 2;
            if Self::check(&s, mid) {
                l = mid;
            } else {
                r = mid - 1;
            }
        }
        s.truncate(l);
        return s;
    }
}
