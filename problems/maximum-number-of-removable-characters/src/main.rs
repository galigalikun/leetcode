fn main() {
    assert_eq!(
        Solution::maximum_removals("abcacb".to_string(), "ab".to_string(), vec![3, 1, 0]),
        2
    );
    assert_eq!(
        Solution::maximum_removals(
            "abcbddddd".to_string(),
            "abcd".to_string(),
            vec![3, 2, 1, 4, 5, 6]
        ),
        1
    );
    assert_eq!(
        Solution::maximum_removals("abcab".to_string(), "abc".to_string(), vec![0, 1, 2, 3, 4]),
        0
    );
}

struct Solution;
impl Solution {
    fn can_form(s: &[u8], p: &[u8], removable: &[i32], k: i32) -> bool {
        let mut removed = vec![false; s.len()];
        for &index in &removable[..k as usize] {
            removed[index as usize] = true;
        }

        let mut j = 0;
        for i in 0..s.len() {
            if !removed[i] && j < p.len() && s[i] == p[j] {
                j += 1;
            }
        }
        j == p.len()
    }
    pub fn maximum_removals(s: String, p: String, removable: Vec<i32>) -> i32 {
        let s_bytes = s.as_bytes();
        let p_bytes = p.as_bytes();
        let mut left = 0;
        let mut right = removable.len() as i32;

        while left < right {
            let mid = (left + right + 1) / 2;
            if Self::can_form(s_bytes, p_bytes, &removable, mid) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        left
    }
}
