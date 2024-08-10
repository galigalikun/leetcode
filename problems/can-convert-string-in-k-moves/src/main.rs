fn main() {
    assert_eq!(
        Solution::can_convert_string("input".to_string(), "ouput".to_string(), 9),
        true
    );
    assert_eq!(
        Solution::can_convert_string("abc".to_string(), "bcd".to_string(), 10),
        false
    );
    assert_eq!(
        Solution::can_convert_string("aab".to_string(), "bbb".to_string(), 27),
        true
    );
}

struct Solution;
impl Solution {
    pub fn can_convert_string(s: String, t: String, k: i32) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut diff = vec![0; 26];
        for i in 0..s.len() {
            let s = s.as_bytes()[i] as i32 - 'a' as i32;
            let t = t.as_bytes()[i] as i32 - 'a' as i32;
            if s != t {
                let d = (t - s + 26) % 26;
                diff[d as usize] += 1;
            }
        }
        for i in 1..26 {
            let x = (i + 26 * (diff[i] - 1)) as i32;
            if x <= k {
                return true;
            }
        }
        return false;
    }
}
