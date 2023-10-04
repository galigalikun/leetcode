fn main() {
    assert_eq!(Solution::balanced_string_split("RLRRLLRLRL".to_string()), 4);
    assert_eq!(Solution::balanced_string_split("RLRRRLLRLL".to_string()), 2);
    assert_eq!(Solution::balanced_string_split("LLLLRRRR".to_string()), 1);
}

struct Solution;
impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut count = 0;
        let mut result = 0;
        for c in s.chars() {
            if c == 'R' {
                count += 1;
            } else {
                count -= 1;
            }
            if count == 0 {
                result += 1;
            }
        }
        return result;
    }
}
