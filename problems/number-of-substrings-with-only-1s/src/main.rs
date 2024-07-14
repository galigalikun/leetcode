fn main() {
    assert_eq!(Solution::num_sub("0110111".to_string()), 9);
    assert_eq!(Solution::num_sub("101".to_string()), 2);
    assert_eq!(Solution::num_sub("111111".to_string()), 21);
}

struct Solution;
impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let mut count = 0;
        let mut result = 0;
        for c in s.chars() {
            if c == '1' {
                count += 1;
            } else {
                result += count * (count + 1) / 2;
                count = 0;
            }
        }
        result += count * (count + 1) / 2;
        return result;
    }
}
