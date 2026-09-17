fn main() {
    assert_eq!(Solution::min_flips_mono_incr("00110".to_string()), 1);
    assert_eq!(Solution::min_flips_mono_incr("010110".to_string()), 2);
    assert_eq!(Solution::min_flips_mono_incr("00011000".to_string()), 2);
}

struct Solution;
impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let mut ones = 0;
        let mut flips = 0;

        for ch in s.bytes() {
            if ch == b'1' {
                ones += 1;
            } else {
                flips = (flips + 1).min(ones);
            }
        }

        flips
    }
}
