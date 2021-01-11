fn main() {
    assert_eq!(Solution::num_decodings("12".to_string()), 2);
    assert_eq!(Solution::num_decodings("225".to_string()), 3);
    assert_eq!(Solution::num_decodings("0".to_string()), 0);
    assert_eq!(Solution::num_decodings("1".to_string()), 1);
    assert_eq!(Solution::num_decodings("27".to_string()), 1);
}

pub struct Solution {}
// https://www.youtube.com/watch?v=o1i7JYWbwOE
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.as_str().chars().count() == 0 {
            return 0;
        }
        if Some('0') == s.as_str().chars().nth(0) {
            return 0;
        }
        if s.as_str().chars().count() == 1 {
            return 1;
        }
        let mut count1 = 1;
        let mut count2 = 1;
        for i in 1..s.as_str().chars().count() {
            let d = s.as_str().chars().nth(i).unwrap().to_digit(10).unwrap();
            let dd = s.as_str().chars().nth(i-1).unwrap().to_digit(10).unwrap()*10 + d;
            let mut count = 0;
            if d > 0 {
                count += count2;
            }
            if dd >= 10 && dd <= 26 {
                count += count1;
            }
            count1 = count2;
            count2 = count;
        }

        return count2;
    }
}
