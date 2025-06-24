fn main() {
    assert_eq!(Solution::min_flips("111000".to_string()), 2);
    assert_eq!(Solution::min_flips("010".to_string()), 0);
    assert_eq!(Solution::min_flips("1110".to_string()), 1);
    assert_eq!(Solution::min_flips("01001001101".to_string()), 2);
}

struct Solution;
impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let n = s.len();
        let s = s.as_bytes();
        let mut ans = i32::MAX;

        for i in 0..2 {
            let mut cnt = 0;
            for j in 0..n {
                if (s[j] - b'0' + i) % 2 != j as u8 % 2 {
                    cnt += 1;
                }
            }
            ans = ans.min(cnt);
        }

        ans
    }
}
