fn main() {
    assert_eq!(Solution::count_good_substrings("xyzzaz".to_string()), 1);
    assert_eq!(Solution::count_good_substrings("aababcabc".to_string()), 4);
    assert_eq!(Solution::count_good_substrings("x".to_string()), 0);
}

struct Solution;
impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        let mut count = 0;
        let bytes = s.as_bytes();
        let n = bytes.len();
        if n < 3 {
            return 0;
        }
        for i in 0..n - 2 {
            if bytes[i] != bytes[i + 1] && bytes[i] != bytes[i + 2] && bytes[i + 1] != bytes[i + 2]
            {
                count += 1;
            }
        }

        count
    }
}
