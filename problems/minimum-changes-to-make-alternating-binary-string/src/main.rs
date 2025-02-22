fn main() {
    assert_eq!(Solution::min_operations("0100".to_string()), 1);
    assert_eq!(Solution::min_operations("10".to_string()), 0);
    assert_eq!(Solution::min_operations("1111".to_string()), 2);
}

struct Solution;
impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut count1 = 0;
        let mut count2 = 0;
        for (i, c) in s.chars().enumerate() {
            if i % 2 == 0 {
                if c == '0' {
                    count1 += 1;
                } else {
                    count2 += 1;
                }
            } else {
                if c == '0' {
                    count2 += 1;
                } else {
                    count1 += 1;
                }
            }
        }
        if count1 < count2 {
            count1
        } else {
            count2
        }
    }
}
