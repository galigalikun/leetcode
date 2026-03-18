fn main() {
    assert_eq!(Solution::min_operations("110".to_string(), 1), 1);
    assert_eq!(Solution::min_operations("0101".to_string(), 3), 2);
    assert_eq!(Solution::min_operations("101".to_string(), 2), -1);
}

struct Solution;
impl Solution {
    pub fn min_operations(s: String, k: i32) -> i32 {
        let mut count = 0;
        let mut chars = s.chars().collect::<Vec<char>>();
        for i in (0..chars.len()).rev() {
            if chars[i] == '1' {
                count += 1;
                if count > k {
                    break;
                }
            } else {
                chars[i] = '1';
            }
        }
        count
    }
}
