fn main() {
    assert_eq!(Solution::has_all_codes("00110110".to_string(), 2), true);
    assert_eq!(Solution::has_all_codes("0110".to_string(), 1), true);
    assert_eq!(Solution::has_all_codes("0110".to_string(), 2), false);
    assert_eq!(Solution::has_all_codes("0".to_string(), 20), false);
}

struct Solution;
impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let mut set = std::collections::HashSet::new();
        let n = s.len();
        let k = k as usize;
        if n < k {
            return false;
        }
        for i in 0..n-k+1 {
            set.insert(&s[i..i+k]);
        }
        if set.len() == 1 << k {
            return true;
        }
        return false;
    }
}
