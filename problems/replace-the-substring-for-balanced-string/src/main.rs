fn main() {
    assert_eq!(Solution::balanced_string("QWER".to_string()), 0);
    assert_eq!(Solution::balanced_string("QQWE".to_string()), 1);
    assert_eq!(Solution::balanced_string("QQQW".to_string()), 2);
}

struct Solution;
impl Solution {
    pub fn balanced_string(s: String) -> i32 {
        let mut count = [0; 32];
        let mut i = 0;
        let mut j = 0;
        let mut res = s.len() as i32;
        let n = s.len();
        let s = s.as_bytes();
        for c in s {
            count[(c - b'A') as usize] += 1;
        }
        while j < n {
            count[(s[j] - b'A') as usize] -= 1;
            while i < n && count.iter().all(|&x| x <= n / 4) {
                res = res.min(j as i32 - i as i32 + 1);
                count[(s[i] - b'A') as usize] += 1;
                i += 1;
            }
            j += 1;
        }
        return res;
    }
}
