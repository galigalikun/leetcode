fn main() {
    assert_eq!(
        Solution::equal_substring("abcd".to_string(), "bcdf".to_string(), 3),
        3
    );
    assert_eq!(
        Solution::equal_substring("abcd".to_string(), "cdef".to_string(), 3),
        1
    );
    assert_eq!(
        Solution::equal_substring("abcd".to_string(), "acde".to_string(), 0),
        1
    );
}

struct Solution;
impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut cost = 0;
        let mut max = 0;
        let mut left = 0;
        let mut right = 0;
        while right < s.len() {
            cost += (s[right] as i32 - t[right] as i32).abs();
            while cost > max_cost {
                cost -= (s[left] as i32 - t[left] as i32).abs();
                left += 1;
            }
            max = max.max(right as i32 - left as i32 + 1);
            right += 1;
        }
        return max;
    }
}
