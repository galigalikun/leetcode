fn main() {
    assert_eq!(Solution::number_of_substrings("00011".to_string()), 5);
    assert_eq!(Solution::number_of_substrings("101101".to_string()), 16);
}

struct Solution;
impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut count = [0; 3];
        let mut res = 0;
        let mut left = 0;
        for right in 0..n {
            count[(s[right] - b'0') as usize] += 1;
            while count.iter().all(|&c| c > 0) {
                count[(s[left] - b'0') as usize] -= 1;
                left += 1;
            }
            res += left as i32;
        }
        res
    }
}
