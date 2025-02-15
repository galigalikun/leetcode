fn main() {
    assert_eq!(Solution::minimum_length("ca".to_string()), 2);
    assert_eq!(Solution::minimum_length("cabaabac".to_string()), 0);
    assert_eq!(Solution::minimum_length("aabccabba".to_string()), 3);
    assert_eq!(Solution::minimum_length("bbbbbbbbbbbbbbbbbbbbbbbbbbbabbbbbbbbbbbbbbbccbcbcbccbbabbb".to_string()), 1);
}

struct Solution;
impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let s = s.as_bytes();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if s[i] != s[j] {
                return (j - i + 1) as i32;
            }
            let mut k = i + 1;
            while k < j && s[k] == s[i] {
                k += 1;
            }
            let mut l = j - 1;
            while l > k && s[l] == s[j] {
                l -= 1;
            }
            i = k;
            j = l;
        }
        return 0;
    }
}
